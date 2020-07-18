#version 330

uniform mat4 model;
uniform mat4 view_proj;

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;

out vec4 color;
out vec4 normal;

void main() {
  mat4 mvp = view_proj * model;
  gl_Position = mvp * vec4(in_position, 1.0);
  color = vec4(1, 1, 1, 1);
  normal = model * vec4(in_normal, 0.0);
}