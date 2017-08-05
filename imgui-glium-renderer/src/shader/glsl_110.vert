#version 110

uniform mat4 matrix;

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 col;

varying vec2 f_uv;
varying vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = uv;
  f_color = col / 255.0;
  gl_Position = matrix * vec4(pos.xy, 0, 1);
}
