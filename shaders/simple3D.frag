const int LIGHT_COUNT = 1;

uniform vec4 u_light_diffuse[LIGHT_COUNT];
uniform vec4 u_light_ambient[LIGHT_COUNT];
uniform vec4 u_light_specular[LIGHT_COUNT];
uniform vec4 u_material_diffuse;
uniform vec4 u_material_specular;
uniform vec4 u_material_ambient;
uniform float u_material_shininess;
uniform sampler2D texture;

varying vec4 s[LIGHT_COUNT];
varying vec4 v;
varying vec4 normal;
varying vec2 v_uv;

void main(void)
{
	vec4 global_ambient = vec4(0.4, 0.4, 0.4, 1.0);
	vec4 light_calculated_color = vec4(0.0, 0.0, 0.0, 0.0);

	vec4 ambient_material = u_material_ambient * texture2D(texture, v_uv);
	vec4 diffuse_material = u_material_diffuse * texture2D(texture, v_uv);
	for (int i = 0; i < LIGHT_COUNT; i++) {
		vec4 h = normalize(s[i] + v);
    	float lambert = max(0.0, dot(normal, s[i]) / (length(normal) * length(s[i])));
		float phong = max(0.0, dot(normal, h) / length(normal) * length(h));
		vec4 ambient_color = u_light_ambient[i] * ambient_material;
		vec4 diffuse_color = u_light_diffuse[i] * diffuse_material * lambert;
		vec4 specular_color = u_light_specular[i] * u_material_specular * pow(phong, u_material_shininess);
		light_calculated_color += ambient_color + diffuse_color + specular_color;
	}

    gl_FragColor = global_ambient * ambient_material + light_calculated_color;
}