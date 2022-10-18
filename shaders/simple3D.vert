const int LIGHT_COUNT = 1;

attribute vec3 a_position;
attribute vec3 a_normal;
uniform vec4 u_light_position[LIGHT_COUNT];
uniform vec4 u_eye_position;

uniform mat4 u_model_matrix;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;

varying vec4 s[LIGHT_COUNT];
varying vec4 v;
varying vec4 normal;

void main(void)
{
	vec4 position = vec4(a_position.x, a_position.y, a_position.z, 1.0);
	normal = vec4(a_normal.x, a_normal.y, a_normal.z, 0.0);

	position = u_model_matrix * position;
	normal = u_model_matrix * normal;

	v = u_eye_position - position;
	for (int i = 0; i < LIGHT_COUNT; i++) {
		s[i] = u_light_position[i].w == 1.0 ? u_light_position[i] - position : u_light_position[i];
	}

	position = u_view_matrix * position;
	position = u_projection_matrix * position;

	gl_Position = position;
}