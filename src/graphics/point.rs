use crate::graphics::*;
use crate::math::*;
use crate::*;
use std::mem::size_of;

pub fn draw_point(ctx: &mut Ctx, pos: Vec3, color: Vec4) {
  let i = ctx.gfx.points.count;

  // TODO debug only?
  if i >= MAX_POINTS {
    panic!("can't draw that many points!")
  }

  ctx.gfx.points.count += 1;

  ctx.gfx.points.e[i] = DrawPoint::new(pos.x(), pos.y(), pos.z(), color);
}

pub fn init(ctx: &mut Ctx) {
  let (vs_src, fs_src) = match sg_api() {
    SgApi::OpenGL33 => (
      include_str!("point.vert.glsl"),
      include_str!("point.frag.glsl"),
    ),
    SgApi::Metal => (
      include_str!("point.vs.metal"),
      include_str!("point.fs.metal"),
    ),
    _ => panic!(),
  };

  let shader_desc = SgShaderDesc {
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
  };

  let shader = sg_make_shader(&shader_desc);

  let layout = SgLayoutDesc {
    attrs: vec![
      SgVertexAttrDesc {
        // name: "in_position",
        format: SgVertexFormat::Float4,
        ..Default::default()
      },
      SgVertexAttrDesc {
        // name: "in_color",
        format: SgVertexFormat::Float4,
        ..Default::default()
      },
    ],
    ..Default::default()
  };

  let point_pipeline_desc = SgPipelineDesc {
    primitive_type: SgPrimitiveType::Points,
    shader,
    layout,
    ..Default::default()
  };

  let pipeline = sg_make_pipeline(&point_pipeline_desc);

  let vertex_buffer_desc = SgBufferDesc {
    buffer_type: SgBufferType::VertexBuffer,
    size: BYTES_POINTS,
    usage: SgUsage::Stream,
  };

  let vertex_buffer = sg_make_buffer::<()>(None, &vertex_buffer_desc);

  let vertex_buffers = vec![vertex_buffer];

  let bindings = SgBindings {
    vertex_buffers,
    ..Default::default()
  };

  ctx.gfx.points.shape = GlShape { bindings, pipeline };
}

pub fn present(ctx: &mut Ctx) {
  sg_update_buffer(
    ctx.gfx.points.shape.bindings.vertex_buffers[0],
    &ctx.gfx.points.e,
    (ctx.gfx.points.count * size_of::<DrawPoint>()) as i32,
  );
  sg_apply_pipeline(ctx.gfx.points.shape.pipeline);
  sg_apply_bindings(&ctx.gfx.points.shape.bindings);
  sg_apply_uniforms(
    SgShaderStage::Vertex,
    0,
    &ctx.gfx.view_proj,
    size_of::<Mat4>() as i32,
  );
  sg_draw(0, ctx.gfx.points.count as i32, 1);
}
