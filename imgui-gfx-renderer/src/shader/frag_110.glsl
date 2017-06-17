#version 110

uniform sampler2D tex;

varying vec2 f_uv;
varying vec4 f_color;

void main() {
  gl_FragColor = f_color * texture2D(tex, f_uv.st);
}
