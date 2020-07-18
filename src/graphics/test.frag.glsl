#version 330

in vec4 color;
in vec4 normal;
out vec4 frag_color;

void main() {
  vec4 normalized_normal = normalize(normal);
  float min = 0.5;
  float max = 1.0;
  float percent = (normalized_normal.z + 1) * 0.5;
  float upish = mix(min, max, percent);
  frag_color = vec4(upish, upish, upish, 1);
}