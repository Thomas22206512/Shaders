#version 100
precision lowp float;

varying vec2 uv;

uniform float iTime;
uniform vec2 iResolution;

float mandelbrot(vec2 c) {
    vec2 z = vec2(0.0);
    int max_iterations = 100;
    int i;
    for (i = 0; i < max_iterations; i++) {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (2.0 * z.x * z.y) + c.y;

        if ((x * x + y * y) > 4.0) break;

        z.x = x;
        z.y = y;
    }

    return float(i) / float(max_iterations);
}

void main() {
    vec2 p = (gl_FragCoord.xy / iResolution.xy) * 2.0 - 1.0;
    p.x *= iResolution.x / iResolution.y; // correction aspect ratio

    float zoom = 1.5 + sin(iTime * 0.2) * 0.5;
    vec2 center = vec2(-0.745, 0.186);
    vec2 c = center + p * zoom;

    float m = mandelbrot(c);

    vec3 color = vec3(m);
    color = mix(vec3(0.0, 0.1, 0.3), vec3(0.8, 0.9, 1.0), m);

    gl_FragColor = vec4(color, 1.0);
}
