#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec4 outColor;

void main() {
    outColor = vec4(color, 1);
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
