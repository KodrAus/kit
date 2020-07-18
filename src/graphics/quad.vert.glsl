#version 410 core

uniform mat4 projection;
layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_uv;

out vec2 uv;
out vec4 debug_color;

void
main() {
  uv = in_uv;
  gl_Position = projection * (vec4(in_position, 1.0));
  float z = in_position.z;
  debug_color = vec4(z, z, z, 1.0);
}
