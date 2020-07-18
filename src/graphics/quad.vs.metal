// #include <metal_stdlib>
// using namespace metal; 

// // layout(location = 0) in vec3 in_position;
// struct vs_in { 
//   float4 position [[attribute(0)]];
//   float2 uv [[attribute(1)]];
// };
// struct vs_out {
//   float4 pos [[position]];
//   float2 uv;
//   float4 color;
// };
// vertex vs_out _main(vs_in in [[stage_in]], constant params_t& params [[buffer(0)]]) {
//   vs_out out;
//   out.pos = params.mvp * in.position;
//   out.uv = in.uv;
//   // float z = in_position.z;
//   // debug_color = vec4(z, z, z, 1.0);
//   // out.color = in.color;
//   out.color = float4(1.0, 0.0, 0.0, 1.0);

//   return out;
// }


#include <metal_stdlib>
using namespace metal;
struct params_t {
  float4x4 mvp;
}; 
struct vs_in {
    float4 position [[attribute(0)]];
    float2 uv [[attribute(1)]];
};
struct vs_out {
    float4 position [[position]];
    float2 uv;
};
vertex vs_out _main(vs_in in [[stage_in]], constant params_t& params [[buffer(0)]]) {
    vs_out out;
    out.position = params.mvp * in.position;
    out.uv = in.uv;
    return out;
}