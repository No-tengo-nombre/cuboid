#version 330 core

in vec3 outColor;

out vec4 final_color;

void main() {
    final_color = vec4(outColor, 1.0);
}
