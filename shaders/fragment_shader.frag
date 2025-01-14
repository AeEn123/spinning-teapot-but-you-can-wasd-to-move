#version 130

in vec3 v_normal;
out vec4 color;

uniform vec3 u_light;
uniform vec3 colour;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = colour*0.6;
    vec3 regular_color = colour;
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}