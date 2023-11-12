#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
float near = 0.1;
float far = 100.0;

uniform sampler2D texture1;

void main()
{
    float ndc = gl_FragCoord.z * 2.0 - 1.0;
    // transforms the non-linear z-buffer (1/z - 1/znear)/1  /  (1/zfar - 1/znear)  to its original linear value (z - znear)/zfar-znear
    // https://www.songho.ca/opengl/gl_projectionmatrix.html
    float linear_depth = (2.0 * near * far) / (far + near - ndc * (far - near));

    FragColor = vec4(vec3(linear_depth / far), 1.0);
}
