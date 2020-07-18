#version 410 core

in vec4 color;

out vec4 final_color;

void main() {
  final_color = color;

  // ramp alpha to 0 if we're at the edge
  vec2 coord = gl_PointCoord - vec2(0.5);
  float axis = max(abs(coord.x), abs(coord.y)) * 2.0;
  float alpha = smoothstep(0.0, 1.0, axis);	
  final_color.a = mix(final_color.a, 0.0, alpha);
}
