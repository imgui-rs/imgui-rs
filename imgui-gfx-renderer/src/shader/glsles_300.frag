#version 300 es

uniform sampler2D tex;

in mediump vec2 f_uv;
in lowp vec4 f_color;

out lowp vec4 Target0;

void main() {
  Target0 = f_color * texture(tex, f_uv.st);
}
