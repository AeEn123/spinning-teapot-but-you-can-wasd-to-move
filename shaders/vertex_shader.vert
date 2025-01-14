#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat4 yaw_matrix;


void main() {
    mat4 modelview = view * model * yaw_matrix;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
}
