#version 110

uniform sampler2D tex;

varying vec2 f_uv;
varying vec4 f_color;

// Built-in:
// vec4 gl_FragColor

void main() {
  gl_FragColor = f_color * texture2D(tex, f_uv.st);
}
