#version 330

in vec2 v2f_UV;
in vec4 v2f_Color;

uniform sampler2D u_FontTexture;

layout(location = 0) out vec4 out_Color;

void main() {
    vec4 tex = texture(u_FontTexture, v2f_UV);
    out_Color = v2f_Color * tex;
}
