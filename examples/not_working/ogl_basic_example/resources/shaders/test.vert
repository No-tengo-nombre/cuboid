#version 420 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

layout (std140, binding = 0) uniform View
{
    mat4 view;
};

uniform vec4 timeColor;

out vec4 outColor;

void main() {
    outColor = vec4(color * timeColor.xyz, timeColor.w);
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0) * view;
}
