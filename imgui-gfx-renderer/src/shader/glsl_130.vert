#version 130

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
  float gamma = 2.2;
  f_color = vec4(
    pow(col.rgb, vec3(gamma)),
    1.0 - pow(1.0 - col.a, gamma)
  );
  gl_Position = matrix * vec4(pos.xy, 0, 1);
}
