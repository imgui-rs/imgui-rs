#version 300 es

uniform mat4 matrix;

in mediump vec2 pos;
in mediump vec2 uv;
in lowp vec4 col;

out mediump vec2 f_uv;
out lowp vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = uv;
  f_color = col / 255.0;
  gl_Position = matrix * vec4(pos.xy, 0, 1);
}
