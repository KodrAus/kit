#version 410 core

in vec2 uv;
in vec4 debug_color;

out vec4 final_color;

uniform sampler2D our_texture;

void main()
{ 
  final_color = texture(our_texture, uv);
  if (final_color.a <= 0.01) {
    discard;
  }
}
