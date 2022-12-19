#version 450

layout(location = 0) in vec2 in_Position;
layout(location = 1) in vec2 in_UV;
layout(location = 2) in vec4 in_Color;

uniform mat4 u_Matrix;

out vec2 v2f_UV;
out vec4 v2f_Color;

void main() {
    gl_Position = u_Matrix * vec4(in_Position, 0.0, 1.0);
    v2f_UV = in_UV;
    v2f_Color = in_Color;
}
