//     // model pipeline
//     {
//         let v_buff_desc = SgBufferDesc {
//             buffer_type: SgBufferType::VertexBuffer,
//             size: size_of::<ModelVert>() * MAX_MODEL_VERTS,
//             usage: SgUsage::Dynamic,
//         };
//         ctx.model_shape.bindings.vertex_buffers[0] = sg_make_buffer::<()>(None, &v_buff_desc);

//         let i_buff_desc = SgBufferDesc {
//             buffer_type: SgBufferType::IndexBuffer,
//             size: size_of::<u32>() * MAX_MODEL_VERTS,
//             usage: SgUsage::Dynamic,
//         };
//         ctx.model_shape.bindings.index_buffer = sg_make_buffer::<()>(None, &i_buff_desc);

//         let shader = sg_make_shader(&SgShaderDesc {
//             vs: SgShaderStageDesc {
//                 source: Some(include_str!("../data/shaders/test.vert.glsl")),
//                 uniform_blocks: vec![SgShaderUniformBlockDesc {
//                     size: size_of::<MeshVsParams>() as i32,
//                     uniforms: vec![
//                         SgShaderUniformDesc {
//                             name: "model",
//                             uniform_type: SgUniformType::Mat4,
//                             ..Default::default()
//                         },
//                         SgShaderUniformDesc {
//                             name: "view_proj",
//                             uniform_type: SgUniformType::Mat4,
//                             ..Default::default()
//                         },
//                     ],
//                 }],
//                 ..Default::default()
//             },
//             fs: SgShaderStageDesc {
//                 source: Some(include_str!("../data/shaders/test.frag.glsl")),
//                 ..Default::default()
//             },
//             attrs: vec![],
//         });
//         let pipeline_desc = SgPipelineDesc {
//             // primitive_type: SG_PRIMITIVETYPE_LINES,
//             shader: shader,
//             index_type: SgIndexType::UInt32,
//             layout: SgLayoutDesc {
//                 attrs: vec![
//                     SgVertexAttrDesc {
//                         format: SgVertexFormat::Float3,
//                         ..Default::default()
//                     }, // in_position
//                     SgVertexAttrDesc {
//                         format: SgVertexFormat::Float3,
//                         ..Default::default()
//                     }, // in_normal
//                        // [1] = {name: "in_uv", format: SgVertexFormat::Float2},
//                        // [3] = {name: "in_color", format: SgVertexFormat::Float4},
//                 ],
//                 ..Default::default()
//             },
//             depth_stencil: SgDepthStencilState {
//                 depth_compare_func: SgCompareFunc::LessEqual,
//                 depth_write_enabled: true,
//                 ..Default::default()
//             },
//             // blend: {
//             //   enabled: true,
//             //   color_format: SgPixelFormat::RGBA8,
//             //   depth_format: SgPixelFormat::Depth,
//             //   dst_factor_rgb: SgBlendFactor::OneMinusSrcAlpha,
//             // },
//             ..Default::default()
//         };
//         ctx.model_shape.pipeline = sg_make_pipeline(&pipeline_desc);
//     }
// }

pub fn present(ctx: &mut Ctx) {
  let model_shape_v_buff = ctx.gl.model_shape.bindings.vertex_buffers[0];
  let model_shape_i_buff = ctx.gl.model_shape.bindings.index_buffer;

  // TODO we should only need to do this step when loading a model
  // let buff = ctx.gl.mesh_buffer;
  // memory_arena_t v = buff.vertex_memory;
  // memory_arena_t i = buff.index_memory;
  // sg_update_buffer(model_shape.bindings.vertex_buffers[0], v.base, v.size);
  // sg_update_buffer(model_shape.bindings.index_buffer, i.base, i.size);

  // sg_apply_pipeline(model_shape.pipeline);
  // sg_apply_bindings(&model_shape.bindings);

  // mesh_vs_params_t mesh_vs_params = {};
  // mesh_vs_params.view_proj = view_proj;

  // for i in 0..ctx.gl.num_meshes
  // {
  //   DrawMesh draw_mesh = ctx.gl.meshes[i];
  //   u8 mesh_i = draw_mesh.mesh_i;
  //   mesh_record_t mesh = ctx.gl.mesh_buffer.meshes[mesh_i];
  //   mesh_vs_params.model = draw_mesh.transform;
  //   sg_apply_uniforms(SG_SHADERSTAGE_VS, 0, &mesh_vs_params, size_of::<mesh_vs_params_t>());
  //   sg_draw(mesh.index_offset, mesh.index_count, 1);
  // }
}
