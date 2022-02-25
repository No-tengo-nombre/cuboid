#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

uniform vec4 timeColor;
uniform mat4 view;

out vec4 outColor;

void main() {
    outColor = vec4(color * timeColor.xyz, timeColor.w);
    // gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    gl_Position = view * vec4(pos.x, pos.y, pos.z, 1.0);
}