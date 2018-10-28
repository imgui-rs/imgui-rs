#version 150

uniform sampler2D tex;

in vec2 f_uv;
in vec4 f_color;

out vec4 out_color;

void main() {
  out_color = f_color * texture(tex, f_uv.st);
}
