#version 330 core
out vec4 FragColor;
 
in vec2 uv;
in vec4 color;

uniform sampler2D tex;

void main()
{
    FragColor = texture(tex, uv) * color;
}
