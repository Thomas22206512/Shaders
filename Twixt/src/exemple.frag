#version 130

uniform int iFrame; // Uniforme pour le numéro de frame
uniform vec2 iResolution; // Résolution de l'écran

out vec4 FragColor;

void main() {
    // Calculer un facteur de temps basé sur iFrame
    float time = float(iFrame) / 60.0; // Divise par 60 pour ralentir l'animation

    // Utilisation du sin pour obtenir une variation continue de couleurs
    vec3 color = 0.5 + 0.5 * cos(time + vec3(0, 2.0, 4.0)); // Animer la couleur avec la fonction cos

    // Appliquer la couleur calculée à l'écran
    FragColor = vec4(color, 1.0); // Rendre le résultat final
}
