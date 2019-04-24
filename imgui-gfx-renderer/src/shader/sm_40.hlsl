cbuffer Constants : register(b0) {
    float4x4 matrix_;
}

Texture2D tex;
SamplerState tex_;

struct VIn {
    float2 position : pos;
    float2 uv : uv;
    float4 color : col;
};

struct VOut
{
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
    float4 color : COLOR;
};

VOut VertexMain(VIn vertex)
{
    VOut output;
    output.position = mul(matrix_, float4(vertex.position, 0.0, 1.0));
    output.uv = vertex.uv;
    output.color = vertex.color;

    return output;
}

float4 PixelMain(VOut vout) : SV_TARGET
{
    return vout.color * tex.Sample(tex_, vout.uv);
}