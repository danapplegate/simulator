#version 330 core

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;

out vec4 FragColor;

uniform vec3 light_color;
uniform vec3 light_pos;
uniform sampler2D textureSource;

void main() {
    float ambientStrength = 0.1;
    vec4 ambient = vec4(ambientStrength * light_color, 1.0);
    vec4 texColor = texture(textureSource, TexCoord);

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light_pos - FragPos);

    float diffStrength = max(dot(norm, lightDir), 0.0);
    vec4 diffuse = diffStrength * vec4(light_color, 1.0);

    FragColor = (ambient + diffuse) * texColor;
}