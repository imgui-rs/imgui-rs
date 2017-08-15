#version 100

uniform mat4 matrix;

attribute mediump vec2 pos;
attribute mediump vec2 uv;
attribute lowp vec4 col;

varying mediump vec2 f_uv;
varying lowp vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = uv;
  f_color = col / 255.0;
  gl_Position = matrix * vec4(pos.xy, 0, 1);
}
