precision mediump float;
varying vec2 tc;

void main() {
    gl_FragColor = vec4(0.0, tc.x, tc.y, 1.0);
}
