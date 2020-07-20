//! Setup and drawing functions related to quads. Typically, you
//! probably don't want to use this directly and instead should use
//! the sprite helper functions, but quads are how the engine thinks
//! about things behind the scenes.

use crate::graphics::*;
use crate::math::*;
use crate::*;
use std::mem::size_of;
use std::mem::size_of_val;

const VERTICES_PER_QUAD: usize = 4;
const INDICES_PER_QUAD: usize = 6;
const MAX_QUAD_VERTS: usize = MAX_QUADS * VERTICES_PER_QUAD;

pub(crate) fn draw_quad(ctx: &mut Ctx, quad: DrawQuad) {
    let i = ctx.gl.quads.count;
    ctx.gl.quads.count += 1;
    ctx.gl.quads.e[i] = quad;
}

pub fn init(ctx: &mut Ctx) {
    let shape = &mut ctx.gl.quads.shape;

    let (vs_src, fs_src) = match sg_api() {
        SgApi::OpenGL33 => (
            include_str!("quad.vert.glsl"),
            include_str!("quad.frag.glsl"),
        ),
        SgApi::Metal => (include_str!("quad.vs.metal"), include_str!("quad.fs.metal")),
        _ => panic!("quad shaders not implemented for this platform"),
    };

    // create a checkerboard texture
    const WIDTH: usize = 4;
    const HEIGHT: usize = 4;
    let pixels_a: [u32; WIDTH] = [0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFF000000];
    let pixels_b: [u32; WIDTH] = [0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF];
    let pixels: [[u32; WIDTH]; HEIGHT] = [pixels_a, pixels_b, pixels_a, pixels_b];
    let debug_image = sg_make_image(
        Some(&[(&pixels, (size_of_val(&pixels)) as i32)]),
        &SgImageDesc {
            width: WIDTH as i32,
            height: HEIGHT as i32,
            ..Default::default()
        },
    );
    shape.bindings.fs_images.push(debug_image);

    shape.bindings.vertex_buffers.push(sg_make_buffer::<()>(
        None,
        &SgBufferDesc {
            buffer_type: SgBufferType::VertexBuffer,
            size: MAX_QUAD_VERTS * size_of::<QuadVert>(),
            usage: SgUsage::Stream,
        },
    ));

    // quad index order never changes, so I can pre-populate it for all possible quads
    let mut indices: [[u16; INDICES_PER_QUAD]; MAX_QUADS] = [[0, 1, 2, 2, 1, 3]; MAX_QUADS];
    for quad_i in 0..MAX_QUADS {
        let offset = (quad_i * VERTICES_PER_QUAD) as u16;
        for i in 0..INDICES_PER_QUAD {
            indices[quad_i][i] += offset;
        }
    }

    shape.bindings.index_buffer = sg_make_buffer(
        Some(&indices),
        &SgBufferDesc {
            buffer_type: SgBufferType::IndexBuffer,
            size: size_of_val(&indices),
            usage: SgUsage::Immutable,
        },
    );

    /* a shader (use separate shader sources here */
    let shd = sg_make_shader(&SgShaderDesc {
        attrs: vec![
            SgShaderAttrDesc {
                name: "in_position",
                ..Default::default()
            },
            SgShaderAttrDesc {
                name: "in_uv",
                ..Default::default()
            },
        ],
        vs: SgShaderStageDesc {
            source: Some(vs_src),
            uniform_blocks: vec![std_uniform_block()],
            ..Default::default()
        },
        fs: SgShaderStageDesc {
            source: Some(fs_src),
            images: vec![SgShaderImageDesc {
                name: "texture",
                image_type: SgImageType::Texture2D,
                ..Default::default()
            }],
            ..Default::default()
        },
    });

    /* a pipeline state object */
    shape.pipeline = sg_make_pipeline(&SgPipelineDesc {
        primitive_type: SgPrimitiveType::Triangles,
        shader: shd,
        index_type: SgIndexType::UInt16,
        layout: SgLayoutDesc {
            // providing attr offsets, but no buffer stride, this should compute the stride
            // TODO I added buffer_index because it was required... it's just set to the default value
            attrs: vec![
                SgVertexAttrDesc {
                    offset: 0,
                    format: SgVertexFormat::Float3,
                    buffer_index: 0,
                },
                SgVertexAttrDesc {
                    offset: 12,
                    format: SgVertexFormat::Float2,
                    buffer_index: 0,
                },
            ],
            ..Default::default()
        },
        blend: SgBlendState {
            enabled: true,
            dst_factor_rgb: SgBlendFactor::OneMinusSrcAlpha,
            src_factor_rgb: SgBlendFactor::SrcAlpha,
            ..Default::default()
        },
        // depth_stencil: SgDepthStencilState {
        //     depth_compare_func: SgCompareFunc::LessEqual,
        //     depth_write_enabled: true,
        //     ..Default::default()
        // },
        // rasterizer: SgRasterizerState {
        //     cull_mode: SgCullMode::Back,
        //     sample_count: SAMPLE_COUNT,
        //     ..Default::default()
        // },
        ..Default::default()
    });
}

pub fn present(ctx: &mut Ctx) {
    let shape = &mut ctx.gl.quads.shape;

    // populate the quad vertex buffer for quad vertices
    // we need to strip out extra info (uniforms) and create a contiguous array of vertices
    // TODO could use vertex stride and
    let mut vertices: [QuadCorners; MAX_QUADS] = [Default::default(); MAX_QUADS];
    for quad_i in 0..ctx.gl.quads.count {
        vertices[quad_i] = ctx.gl.quads.e[quad_i].corners;
    }
    sg_update_buffer(
        shape.bindings.vertex_buffers[0],
        &vertices,
        (ctx.gl.quads.count * size_of::<QuadCorners>()) as i32,
    );

    // draw quad batches
    sg_apply_pipeline(shape.pipeline);
    sg_apply_bindings(&shape.bindings);

    for i in 0..ctx.gl.quads.count {
        let quad = &ctx.gl.quads.e[i];
        let transform = quad.transform;
        let mut mv = ctx.gl.view * transform;

        // cancel out some parts of the model_shape view matrix in order to
        // billboard the sprite
        // https://www.geeks3d.com/20140807/billboarding-vertex-shader-glsl/
        // mv.e[0][0] = transform.e[0][0];
        // mv.e[0][1] = 0.0;
        // mv.e[0][2] = 0.0;

        // mv.e[1][0] = 0.0;
        // mv.e[1][1] = transform.e[1][1];
        // mv.e[1][2] = 0.0;

        // mv.e[2][0] = 0.0;
        // mv.e[2][1] = 0.0;
        // mv.e[2][2] = transform.e[2][2];

        let mvp = ctx.gl.proj * mv;
        sg_apply_uniforms(SgShaderStage::Vertex, 0, &mvp, size_of::<Mat4>() as i32);
        let img_id = quad.img_id;
        shape.bindings.fs_images[0] = ctx.gl.images.e[img_id].e;

        sg_apply_bindings(&shape.bindings); // do I need to re-apply this for each draw call?

        // TODO batching would be nice for sequential sprites with the same texture asset
        sg_draw((i * INDICES_PER_QUAD) as i32, INDICES_PER_QUAD as i32, 1);
    }
}
