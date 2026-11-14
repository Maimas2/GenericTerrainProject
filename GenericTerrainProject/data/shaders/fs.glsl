#version 330
out vec4 out_color;

in vec3 v_color;
in vec3 out_normal;
in vec3 cam_direction;
in float height;

void main() {
    out_color = vec4(v_color, 1.0);
    out_color.xyz *= (height/16.)+0.5;
    // out_color.r = 1.;
    // out_color.b = abs(dot(out_normal, cam_direction));
}