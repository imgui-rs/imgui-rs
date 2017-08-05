#version 100

uniform sampler2D tex;

varying mediump vec2 f_uv;
varying lowp vec4 f_color;

// Built-in:
// vec4 gl_FragColor

void main() {
  gl_FragColor = f_color * texture2D(tex, f_uv.st);
}
