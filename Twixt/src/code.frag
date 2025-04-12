#version 100
#extension GL_EXT_gpu_shader4 : enable
precision mediump float;

uniform vec2 iResolution;
uniform float iTime;
uniform vec2 position; // position_x, position_y
uniform vec2 zoom; // width, height
uniform bool smooth;
uniform sampler2D palette; // texture avec la palette
uniform sampler2D Texture;

vec4 get_color(float t, float range, sampler2D pal, int count) {
    float index = mod(t, range) / range;
    return texture2D(pal, vec2(index, 0.5));
}

float modulus2(vec2 z) {
    return dot(z, z);
}

void main() {
    vec2 uv = gl_FragCoord.xy / iResolution.xy;
    vec2 number;
    int max_iterations = 1000;
    vec2 c = vec2(-0.7, 0.27015);
    float color_range = 0.89;
    float color_shift = 0.0;
    bool smooth = true;
    int colors_nb = 32;
    vec2 position = vec2(0.0, 0.0);
    vec2 zoom = vec2(3.0,2.0);
    zoom -= 1.0/iTime;
    // position dans l’espace fractal
    number.x = uv.x * zoom.x + position.x - zoom.x / 2.0;
    number.y = (1.0 - uv.y) * zoom.y + position.y - zoom.y / 2.0;

    vec2 temp;
    float smooth_value = exp(-length(number));
    float color_mod = float(max_iterations) * 0.01 * color_range * iTime;

    float max_modulus = smooth ? 1000.0 : 4.0;

    int i = 0;
    while (modulus2(number) < max_modulus && i < max_iterations) {
        temp = number;
        number.x = temp.x * temp.x - temp.y * temp.y + c.x;
        number.y = 2.0 * temp.x * temp.y + c.y;
        i++;
        smooth_value += exp(-length(number));
    }

    float color_idx = 0.0;
    if (i == max_iterations) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        return;
    } else if (colors_nb == -1) {
        color_idx = float(i % 6);
        gl_FragColor = get_color(color_idx, 60.0, Texture, 6);
    } else if (colors_nb == -2) {
        color_idx = float(i % 2);
        gl_FragColor = get_color(color_idx, 20.0, Texture, 2);
    } else if (smooth) {
        float shifted_smooth_value = mod(floor(smooth_value) + color_shift * (float(max_iterations) / color_mod), float(max_iterations)) + fract(smooth_value);
        gl_FragColor = get_color(mod(shifted_smooth_value, float(max_iterations) / color_mod), float(max_iterations) / color_mod, Texture, colors_nb);
    } else {
        int shifted_i = int(float(i) + color_shift * (float(max_iterations) / color_mod)) % max_iterations;
        gl_FragColor = get_color(float(shifted_i % int(float(max_iterations) / color_mod)), float(max_iterations) / color_mod + iTime, Texture, colors_nb);
    }
}
