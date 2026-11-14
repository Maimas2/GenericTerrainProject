#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec3 aNormal;

out vec3 v_color;
out vec3 out_normal;
out vec3 cam_direction;
out float height;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 final_transform;
uniform float x_stretch;
uniform vec3 camera_direction;

void main() {
    // vec3 nposition = vec3(position.xy, 1.);
    gl_Position = (perspective * view * final_transform * vec4(position, 1.)) / vec4(x_stretch, 1., 1., 1.);
    
    v_color = color;
    cam_direction = camera_direction;
    out_normal = aNormal;
    height = position.y;
}