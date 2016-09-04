#version 120

attribute vec2 coord3d;
attribute vec3 v_color;

uniform mat4 m_transform;

varying vec3 f_color;

void main() {
    gl_Position = m_transform * vec4(coord3d, 0.0, 1.0);
    f_color = v_color;
}
