#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 vTexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
    FragColor = mix(texture(texture1, vTexCoord), texture(texture2, vTexCoord), texture(texture2, vTexCoord).a * 0.2 );
}