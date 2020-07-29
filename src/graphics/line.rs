use crate::graphics::*;
use crate::math::*;
use crate::*;
use std::mem::size_of;

pub fn draw_line(ctx: &mut Ctx, point_a: Vec3, point_b: Vec3, color: Vec4) {
  let i = ctx.gfx.lines.count;
  ctx.gfx.lines.count += 1;
  // TODO this is essentially 2 vertices. Could do this with a mesh?
  ctx.gfx.lines.e[i] = DrawLine {
    point_a: point_a.extend(1.0),
    color_a: color,
    point_b: point_b.extend(1.0),
    color_b: color,
  };
}

pub fn init(ctx: &mut Ctx) {
  // TODO common primitives (line, point, maybe others?) could share the same shaders
  let (vs_src, fs_src) = match sg_api() {
    SgApi::OpenGL33 => (
      include_str!("line.vert.glsl"),
      include_str!("line.frag.glsl"),
    ),
    SgApi::Metal => (include_str!("line.vs.metal"), include_str!("line.fs.metal")),
    _ => panic!(),
  };

  let pipeline = sg_make_pipeline(&SgPipelineDesc {
    // index_type: SgIndexType::UInt32,
    primitive_type: SgPrimitiveType::Lines, // TODO replace with line strip?
    shader: sg_make_shader(&SgShaderDesc {
      vs: SgShaderStageDesc {
        source: Some(vs_src),
        uniform_blocks: vec![std_uniform_block()],
        ..Default::default()
      },
      fs: SgShaderStageDesc {
        source: Some(fs_src),
        ..Default::default()
      },
      attrs: vec![],
    }),
    layout: SgLayoutDesc {
      attrs: vec![
        SgVertexAttrDesc {
          // name : "in_position",
          format: SgVertexFormat::Float4,
          ..Default::default()
        },
        SgVertexAttrDesc {
          // name : "in_color",
          format: SgVertexFormat::Float4,
          ..Default::default()
        },
      ],
      ..Default::default()
    },
    // depth_stencil: SgDepthStencilState {
    //     depth_compare_func: SgCompareFunc::LessEqual,
    //     depth_write_enabled: true,
    //     ..Default::default()
    // },
    // blend: SgBlendState {
    //     enabled: true,
    //     color_format: SgPixelFormat::RGBA8,
    //     depth_format: SgPixelFormat::Depth,
    //     dst_factor_rgb: SgBlendFactor::OneMinusSrcAlpha,
    //     ..Default::default()
    // },
    ..Default::default()
  });

  let bindings = SgBindings {
    vertex_buffers: vec![sg_make_buffer::<()>(
      None,
      &SgBufferDesc {
        size: BYTES_LINES,
        usage: SgUsage::Stream,
        ..Default::default()
      },
    )],
    ..Default::default()
  };

  ctx.gfx.lines.shape = GlShape { bindings, pipeline };
}

pub fn present(ctx: &mut Ctx) {
  sg_update_buffer(
    ctx.gfx.lines.shape.bindings.vertex_buffers[0],
    &ctx.gfx.lines.e,
    (ctx.gfx.lines.count * size_of::<DrawLine>()) as i32,
  );

  sg_apply_pipeline(ctx.gfx.lines.shape.pipeline);
  sg_apply_bindings(&ctx.gfx.lines.shape.bindings);

  sg_apply_uniforms(
    SgShaderStage::Vertex,
    0,
    &ctx.gfx.view_proj,
    size_of::<Mat4>() as i32,
  );

  sg_draw(0, (ctx.gfx.lines.count * 2) as i32, 1);
}
