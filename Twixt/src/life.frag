#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;        // Texture d'entrée (état précédent)
uniform vec2 texel_size;          // Taille d’un pixel dans l’image (1.0 / resolution)
uniform float time;               // Temps (facultatif)
uniform vec2 resolution;          // Résolution (taille de la texture ou écran)

#define VARIANT 1

// Lecture de cellule voisine (avec wrap-around)
int cell(vec2 offset) {
    vec2 coord = mod(uv + offset * texel_size, vec2(1.0)); // wrap-around
    float value = texture2D(Texture, coord).r;
    return (value > 0.5) ? 1 : 0;
}

float hash1(float n) {
    return fract(sin(n) * 138.5453123);
}

void main() {
    int k = 0;
    int e = cell(vec2(0.0)); // valeur actuelle

#if VARIANT == 0
    k = cell(vec2(-1.0, -1.0)) + cell(vec2(0.0, -1.0)) + cell(vec2(1.0, -1.0)) +
        cell(vec2(-1.0,  0.0))                         + cell(vec2(1.0,  0.0)) +
        cell(vec2(-1.0,  1.0)) + cell(vec2(0.0,  1.0)) + cell(vec2(1.0,  1.0));
    float f = ((k == 2 && e == 1) || k == 3 || k == 7) ? 1.0 : 0.0;
#endif

#if VARIANT == 1
    k = cell(vec2(-1.0, -1.0)) + cell(vec2(0.0, -1.0)) + cell(vec2(1.0, -1.0)) +
        cell(vec2(-1.0,  0.0)) + e * 9                 + cell(vec2(1.0,  0.0)) +
        cell(vec2(-1.0,  1.0)) + cell(vec2(0.0,  1.0)) + cell(vec2(1.0,  1.0));
    float f = (k == 3 || k == 11 || k == 12) ? 1.0 : 0.0;
#endif

#if VARIANT == 2
    k =  cell(vec2(-1.0, -1.0)) +  cell(vec2(0.0, -1.0)) + cell(vec2(1.0, -1.0)) +
         cell(vec2(-1.0,  0.0)) - 9 * e                 + cell(vec2(1.0,  0.0)) +
         cell(vec2(-1.0,  1.0)) +  cell(vec2(0.0,  1.0)) + cell(vec2(1.0,  1.0));
    float f = (k == -7 || k == -6 || k == 3) ? 1.0 : 0.0;
#endif

    // Initialisation pseudo-aléatoire
    if (time < 0.1) {
        float rnd = hash1(gl_FragCoord.x * 13.0 + hash1(gl_FragCoord.y * 71.1));
        f = step(0.9, rnd);
    }

    gl_FragColor = vec4(f, 0.0, 0.0, 1.0);
}
