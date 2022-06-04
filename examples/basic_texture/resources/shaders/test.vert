#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

layout (std140, binding = 0) uniform View
{
    mat4 view;
};

out vec4 outColor;
out vec2 texCoord;

void main() {
    outColor = vec4(aColor, 1.0);
    gl_Position = vec4(aPos, 1.0) * view;
    texCoord = aTexCoord.xy;
}
