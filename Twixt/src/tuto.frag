#version 100
precision lowp float;

varying vec2 uv;
uniform sampler2D Texture;
uniform vec2 texel_size;
uniform float time; // ⏱️ uniform envoyé depuis Rust

void main() {
    vec4 color = vec4(0.0);

    float blur_strength = 1.0 + sin(time * 2.0) * 3.0;

    vec2 center_min = vec2(0.2, 0.2);
    vec2 center_max = vec2(0.6, 0.6);

    // Si on est dans la zone "propre", pas de flou
    if (uv.x > center_min.x && uv.x < center_max.x &&
        uv.y > center_min.y && uv.y < center_max.y) {
        gl_FragColor = texture2D(Texture, uv);
        return;
    }
    // blur 5x5
    float weight = 1.0 / 25.0;
    for (int x = -2; x <= 2; x++) {
        for (int y = -2; y <= 2; y++) {
            vec2 offset = vec2(float(x), float(y)) * texel_size * blur_strength;
            color += texture2D(Texture, uv + offset) * weight;
        }
    }
    
    gl_FragColor = fract(color);
}
