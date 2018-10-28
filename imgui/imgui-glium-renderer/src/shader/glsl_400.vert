#version 400

uniform mat4 matrix;

in vec2 pos;
in vec2 uv;
in vec4 col;

out vec2 f_uv;
out vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = uv;
  f_color = col / 255.0;
  gl_Position = matrix * vec4(pos.xy, 0, 1);
}
