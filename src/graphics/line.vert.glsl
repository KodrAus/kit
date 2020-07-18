#version 410 core

uniform mat4 projection;
layout (location = 0) in vec3 in_position;
layout (location = 1) in vec4 in_color;

out vec4 color;

void main() {
  gl_Position = projection * vec4(in_position, 1.0);
  color = in_color;
}
