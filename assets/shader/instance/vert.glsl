#version 460 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;

layout(location = 2) in mat4 iModel;
layout(location = 6) in vec4 iColor;

uniform mat4 uProjection;

out vec4 fColor;

void main() {
    gl_Position = uProjection * iModel * vec4(aPos, 1.0);
    fColor = iColor;
}
