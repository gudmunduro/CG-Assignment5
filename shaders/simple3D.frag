const int LIGHT_COUNT = 1;

uniform vec4 u_light_diffuse[LIGHT_COUNT];
uniform vec4 u_light_ambient[LIGHT_COUNT];
uniform vec4 u_light_specular[LIGHT_COUNT];
uniform vec4 u_material_diffuse;
uniform vec4 u_material_specular;
uniform float u_material_ambient_factor;
uniform float u_material_shininess;

varying vec4 s[LIGHT_COUNT];
varying vec4 v;
varying vec4 normal;

void main(void)
{
	vec4 global_ambient = vec4(0.4, 0.4, 0.4, 1.0);
	vec4 material_ambient = u_material_ambient_factor * u_material_diffuse;
	vec4 light_calculated_color = vec4(0.0, 0.0, 0.0, 0.0);

	for (int i = 0; i < LIGHT_COUNT; i++) {
		vec4 h = normalize(s[i] + v);
    	float lambert = max(0.0, dot(normal, s[i]) / (length(normal) * length(s[i])));
		float phong = max(0.0, dot(normal, h) / length(normal) * length(h));
		vec4 ambient_color = u_light_ambient[i] * material_ambient;
		vec4 diffuse_color = u_light_diffuse[i] * u_material_diffuse * lambert;
		vec4 specular_color = u_light_specular[i] * u_material_specular * pow(phong, u_material_shininess);
		light_calculated_color += ambient_color + diffuse_color + specular_color;
	}

    gl_FragColor = global_ambient * material_ambient + light_calculated_color;
}