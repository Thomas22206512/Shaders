uniform float iTime;  // Le temps depuis que le shader a démarré

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    float time = iTime;  // Utilisation du temps

    // Ton code de shader ici, par exemple :
    fragColor = vec4(sin(time), cos(time), 0.5, 1.0);
}