use crate::*;
use std::mem;
use std::mem::size_of_val;

const SAMPLE_COUNT: i32 = 4;

pub(crate) struct MeshVert {
  pub pos: Vec3,
  //   pub uv: Vec2,
  //   pub normal: Vec3, // TODO store and use these to smooth faces
  pub color: Vec4,
}

// struct Model {
//   v_offset: size_t,
//   i_offset: size_t,
//   v_count: u32,
//   i_count: u32,
//   // TODO textures
// } model_t,

pub fn draw_mesh(ctx: &mut Ctx, mesh_i: usize, transform: Mat4) {
  let mesh = &mut ctx.gfx.mesh;

  let i = mesh.count;

  if i == MAX_MESHES {
    panic!("Too many mesh draw calls! Maximum is {}", MAX_MESHES);
  }

  mesh.count += 1;

  mesh.e[i] = DrawMesh { mesh_i, transform };
}

pub fn add_cube_mesh(ctx: &mut Ctx) {}

pub fn init(ctx: &mut Ctx) {
  let shape = &mut ctx.gfx.mesh.shape;

  // cube vertex buffer
  let vertices: [MeshVert; 24] = [
    MeshVert {
      pos: vec3(-0.5, -0.5, -0.5),
      color: vec4(1.0, 0.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, -0.5),
      color: vec4(1.0, 0.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, -0.5),
      color: vec4(1.0, 0.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, -0.5),
      color: vec4(1.0, 0.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, -0.5, 0.5),
      color: vec4(0.0, 1.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, 0.5),
      color: vec4(0.0, 1.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, 0.5),
      color: vec4(0.0, 1.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, 0.5),
      color: vec4(0.0, 1.0, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, -0.5, -0.5),
      color: vec4(0.0, 0.0, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, -0.5),
      color: vec4(0.0, 0.0, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, 0.5),
      color: vec4(0.0, 0.0, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, -0.5, 0.5),
      color: vec4(0.0, 0.0, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, -0.5),
      color: vec4(1.0, 0.5, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, -0.5),
      color: vec4(1.0, 0.5, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, 0.5),
      color: vec4(1.0, 0.5, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, 0.5),
      color: vec4(1.0, 0.5, 0.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, -0.5, -0.5),
      color: vec4(0.0, 0.5, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, -0.5, 0.5),
      color: vec4(0.0, 0.5, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, 0.5),
      color: vec4(0.0, 0.5, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, -0.5, -0.5),
      color: vec4(0.0, 0.5, 1.0, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, -0.5),
      color: vec4(1.0, 0.0, 0.5, 1.0),
    },
    MeshVert {
      pos: vec3(-0.5, 0.5, 0.5),
      color: vec4(1.0, 0.0, 0.5, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, 0.5),
      color: vec4(1.0, 0.0, 0.5, 1.0),
    },
    MeshVert {
      pos: vec3(0.5, 0.5, -0.5),
      color: vec4(1.0, 0.0, 0.5, 1.0),
    },
  ];

  let vbuf = sg_make_buffer(
    Some(&vertices),
    &SgBufferDesc {
      size: size_of_val(&vertices),
      buffer_type: SgBufferType::VertexBuffer,
      usage: SgUsage::Immutable,
    },
  );

  //     ctx.gfx
  //         .test_shape
  //         .bindings
  //         .vertex_buffers
  //         .push(vbuf);

  let indices: [u16; 36] = [
    0, 1, 2, 0, 2, 3, 6, 5, 4, 7, 6, 4, 8, 9, 10, 8, 10, 11, 14, 13, 12, 15, 14, 12, 16, 17, 18,
    16, 18, 19, 22, 21, 20, 23, 22, 20,
  ];

  let ibuf = sg_make_buffer(
    Some(&indices),
    &SgBufferDesc {
      size: size_of_val(&indices),
      buffer_type: SgBufferType::IndexBuffer,
      ..Default::default()
    },
  );

  let (vs_src, fs_src) = match sg_api() {
    SgApi::Metal => (include_str!("mesh.vs.metal"), include_str!("mesh.fs.metal")),
    SgApi::Direct3D11 => (
      "cbuffer params: register(b0) {
                  float4x4 mvp;
                };
                struct vs_in {
                  float4 pos: POS;
                  float4 color: COLOR0;
                };
                struct vs_out {
                  float4 color: COLOR0;
                  float4 pos: SV_Position;
                };
                vs_out main(vs_in inp) {
                  vs_out outp;
                  outp.pos = mul(mvp, inp.pos);
                  outp.color = inp.color;
                  return outp;
                }",
      "float4 main(float4 color: COLOR0): SV_Target0 {
                  return color;
                }",
    ),
    SgApi::OpenGL33 => (
      "#version 330
            uniform mat4 mvp;
            in vec4 position;
            in vec4 color0;
            out vec4 color;
            void main() {
              gl_Position = mvp * position;
              color = color0;
            }",
      "#version 330
            in vec4 color;
            out vec4 frag_color;
            void main() {
              frag_color = color;
            }",
    ),
    _ => panic!("unimplemented graphics backened"),
  };

  let shd = sg_make_shader(&SgShaderDesc {
    attrs: vec![
      SgShaderAttrDesc {
        name: "position",
        sem_name: "POS",
        ..Default::default()
      },
      SgShaderAttrDesc {
        name: "color0",
        sem_name: "COLOR",
        ..Default::default()
      },
    ],
    vs: SgShaderStageDesc {
      source: Some(vs_src),
      uniform_blocks: vec![SgShaderUniformBlockDesc {
        size: 64,
        uniforms: vec![SgShaderUniformDesc {
          name: "mvp",
          uniform_type: SgUniformType::Mat4,
          ..Default::default()
        }],
      }],
      ..Default::default()
    },
    fs: SgShaderStageDesc {
      source: Some(fs_src),
      ..Default::default()
    },
  });

  shape.pipeline = sg_make_pipeline(&SgPipelineDesc {
    layout: SgLayoutDesc {
      buffers: vec![SgBufferLayoutDesc {
        stride: 28,
        ..Default::default()
      }],
      attrs: vec![
        SgVertexAttrDesc {
          format: SgVertexFormat::Float3,
          ..Default::default()
        },
        SgVertexAttrDesc {
          format: SgVertexFormat::Float4,
          ..Default::default()
        },
      ],
      ..Default::default()
    },
    shader: shd,
    index_type: SgIndexType::UInt16,
    depth_stencil: SgDepthStencilState {
      depth_compare_func: SgCompareFunc::LessEqual,
      depth_write_enabled: true,
      ..Default::default()
    },
    rasterizer: SgRasterizerState {
      cull_mode: SgCullMode::Back,
      sample_count: SAMPLE_COUNT,
      ..Default::default()
    },
    ..Default::default()
  });

  shape.bindings = SgBindings {
    vertex_buffers: vec![vbuf],
    index_buffer: ibuf,
    ..Default::default()
  };
}

pub fn present(ctx: &mut Ctx) {
  let shape = &mut ctx.gfx.mesh.shape;

  for i in 0..ctx.gfx.mesh.count {
    let model = ctx.gfx.mesh.e[i].transform;
    let mvp = ctx.gfx.view_proj * model;
    sg_apply_pipeline(shape.pipeline);
    sg_apply_bindings(&shape.bindings);
    sg_apply_uniforms(SgShaderStage::Vertex, 0, &mvp, 64);
    sg_draw(0, 36, 1);
  }
}
