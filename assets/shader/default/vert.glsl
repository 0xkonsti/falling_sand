#version 460 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;

uniform mat4 uModel;
uniform mat4 uProjection;

out vec4 fColor;

void main() {
    gl_Position = uProjection * uModel * vec4(aPos, 1.0);
    fColor = aColor;
}
