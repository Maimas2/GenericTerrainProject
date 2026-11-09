#version 330 core

layout (location = 0) in vec3 position;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 final_transform;

void main() {
    // vec3 nposition = vec3(position.xy, 1.);
    gl_Position = perspective * view * final_transform * vec4(position, 1.);

    gl_Position.xyz = gl_Position.xyz / gl_Position.w;
    gl_Position.w = 1.;
}