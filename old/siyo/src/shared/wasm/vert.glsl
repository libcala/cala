attribute vec3 position;
attribute vec2 texc;
varying vec2 tc;

// uniform mat4 projection_matrix;

//        uniform mat4 camera_matrix;
//        uniform mat4 object_matrix;

void main() {
    gl_Position = /*projection_matrix * / *camera_matrix * object_matrix */ vec4(position, 1.0);
    tc = texc;
}
