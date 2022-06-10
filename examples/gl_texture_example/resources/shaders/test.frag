#version 460 core

in vec4 outColor;
in vec2 texCoord;

out vec4 finalColor;

uniform sampler2D uTexture;

void main() {
    // finalColor = outColor;
    // finalColor = texture(uTexture, texCoord);
    finalColor = texture(uTexture, texCoord) * outColor;
}
