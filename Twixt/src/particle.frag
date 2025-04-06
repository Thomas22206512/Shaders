#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 texel_size;
uniform float time;
uniform vec2 resolution;
uniform vec3 mouse; // x, y = position; z = clic

void main() {
    float t = time + 5.0;
    float z = 6.0;

    const int n = 100; // particules

    vec3 startColor = vec3(0.0, 0.64, 0.2);
    vec3 endColor   = vec3(0.06, 0.35, 0.85);

    float startRadius = 0.84;
    float endRadius   = 1.6;

    float power    = 0.51;
    float duration = 4.0;

    // Position UV centrée autour de la souris 🐭
    vec2 mouse_uv = (mouse.xy / resolution) * 2.0 - 1.0;
    mouse_uv.x *= resolution.x / resolution.y; // aspect ratio

    vec2 v = z * ((2.0 * uv - 1.0) - mouse_uv);

    // Zoom & durée via souris
    if (mouse.z > 0.0) {
        v *= (mouse.y / resolution.y) * 20.0;
        duration = (mouse.x / resolution.x) * 10.0;
    }

    vec3 col = vec3(0.0);
    float dMax = duration;
    float sum = 0.0;

    float evo = (sin(time * 0.01 + 400.0) * 0.5 + 0.5) * 99.0 + 1.0;

    for (int i = 0; i < n; i++) {
        float d = fract(t * power + 48934.4238 * sin(float(i) / evo * 692.7398));
        float angle = 6.2831 * float(i) / float(n);

        float x = d * cos(angle) * duration;
        float y = d * sin(angle) * duration;

        float distRatio = d / dMax;
        float radius = mix(startRadius, endRadius, distRatio);

        vec2 p = v - vec2(x, y);
        float intensity = radius / dot(p, p + 0.001);

        sum += intensity;
        col = mix(col, mix(startColor, endColor, distRatio), intensity / (sum + 0.001));
    }

    sum /= float(n);
    col = normalize(col) * sum;

    sum = clamp(sum, 0.0, 1.0);
    gl_FragColor = vec4(col * sum, 1.0);
}
