const int MAX_LIGHT_COUNT = 10;
const float FOG_MAXDIST = 250.0;
const float FOG_MINDIST = 0.1;
const vec4 FOG_COLOR = vec4(0.89, 0.67, 0.36, 1.0);

uniform vec4 u_light_diffuse[MAX_LIGHT_COUNT];
uniform vec4 u_light_ambient[MAX_LIGHT_COUNT];
uniform vec4 u_light_specular[MAX_LIGHT_COUNT];
uniform float u_light_max_radius[MAX_LIGHT_COUNT];
uniform int u_light_count;
uniform vec4 u_material_diffuse;
uniform vec4 u_material_specular;
uniform vec4 u_material_ambient;
uniform float u_material_shininess;
uniform sampler2D u_texture_diffuse;
uniform sampler2D u_texture_specular;
uniform bool u_diffuse_active;
uniform bool u_specular_active;
uniform bool u_skybox_mode;

varying vec4 s[MAX_LIGHT_COUNT];
varying float dist[MAX_LIGHT_COUNT];
varying vec4 v;
varying vec4 normal;
varying vec2 v_uv;

void main(void)
{
	if (u_skybox_mode) {
		gl_FragColor = mix(FOG_COLOR, texture2D(u_texture_diffuse, v_uv), 0.2);
		return;
	}

	vec4 global_ambient = vec4(0.4, 0.4, 0.4, 1.0);
	vec4 light_calculated_color = vec4(0.0, 0.0, 0.0, 0.0);

	vec4 diffuse_texture = u_diffuse_active ? texture2D(u_texture_diffuse, v_uv) : vec4(1, 1, 1, 1);
	vec4 specular_texture = u_specular_active ? texture2D(u_texture_specular, v_uv) : vec4(1, 1, 1, 1);
	vec4 ambient_material = u_material_ambient * diffuse_texture;
	vec4 diffuse_material = u_material_diffuse * diffuse_texture;
	vec4 specular_material = u_material_specular * specular_texture;
	for (int i = 0; i < u_light_count; i++) {
		float intensity = dist[i] < u_light_max_radius[i] ? 1.0-pow(dist[i] / u_light_max_radius[i], 2.0) : 0.0;
		vec4 h = normalize(s[i] + v);
    	float lambert = max(0.0, dot(normal, s[i]) / (length(normal) * length(s[i]))) * intensity;
		float phong = max(0.0, dot(normal, h) / length(normal) * length(h)) * intensity;
		vec4 ambient_color = u_light_ambient[i] * ambient_material;
		vec4 diffuse_color = u_light_diffuse[i] * diffuse_material * lambert;
		vec4 specular_color = u_light_specular[i] * specular_material * pow(phong, u_material_shininess);
		light_calculated_color += ambient_color + diffuse_color + specular_color;
	}

	vec4 final_color = global_ambient * ambient_material + light_calculated_color;
    
	// Fog
	float dist = length(v);
	float fog_factor = (FOG_MAXDIST - dist) / (FOG_MAXDIST - FOG_MINDIST);
	fog_factor = clamp(fog_factor, 0.2, 1.0);

	gl_FragColor = mix(FOG_COLOR, final_color, fog_factor);
}