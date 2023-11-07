#version 330 core

in vec2 uv;
in vec4 c_mod;

out vec4 FragColor;

uniform sampler2D tex1;

void main()
{
    FragColor = texture(tex1, uv).xxxx * c_mod;
} 