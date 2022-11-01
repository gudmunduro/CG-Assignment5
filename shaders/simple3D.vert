const int MAX_LIGHT_COUNT = 10;

attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec2 a_uv;
uniform vec4 u_light_position[MAX_LIGHT_COUNT];
uniform int u_light_count;
uniform vec4 u_eye_position;

uniform mat4 u_model_matrix;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;


varying vec4 s[MAX_LIGHT_COUNT];
varying float dist[MAX_LIGHT_COUNT];
varying vec4 v;
varying vec4 normal;
varying vec2 v_uv;

void main(void)
{
	vec4 position = vec4(a_position.x, a_position.y, a_position.z, 1.0);
	normal = vec4(a_normal.x, a_normal.y, a_normal.z, 0.0);

	position = u_model_matrix * position;
	normal = u_model_matrix * normal;

	v = u_eye_position - position;
	for (int i = 0; i < u_light_count; i++) {
		s[i] = u_light_position[i].w == 1.0 ? u_light_position[i] - position : u_light_position[i];
		dist[i] = length(s[i]);
	}

	position = u_view_matrix * position;
	position = u_projection_matrix * position;

	v_uv = a_uv;

	gl_Position = position;
}