use std::{fs, mem};

use glow::*;


pub struct SkyboxShader<'a> {
    gl: &'a Context,
    rendering_program_id: NativeProgram,
    position_loc: u32,
    projection_matrix_loc: NativeUniformLocation,
    view_matrix_loc: NativeUniformLocation,
    skybox_texture_loc: NativeUniformLocation,
}

impl<'a> SkyboxShader<'a> {
    pub fn new(gl: &'a Context) -> SkyboxShader {
        unsafe {
            let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
            let shader_file = fs::read_to_string("./shaders/skybox.vert").unwrap();
            gl.shader_source(vert_shader, &shader_file);
            gl.compile_shader(vert_shader);
            let res = gl.get_shader_compile_status(vert_shader);
            if !res {
                let info_log = gl.get_shader_info_log(vert_shader);
                panic!("Couldn't compile vertex shader\nShader compilation Log:\n{info_log}");
            }

            let frag_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
            let shader_file = fs::read_to_string("./shaders/skybox.frag").unwrap();
            gl.shader_source(frag_shader, &shader_file);
            gl.compile_shader(frag_shader);
            let res = gl.get_shader_compile_status(frag_shader);
            if !res {
                let info_log = gl.get_shader_info_log(frag_shader);
                panic!("Couldn't compile fragment shader\nShader compilation Log:\n{info_log}");
            }

            let rendering_program_id = gl.create_program().unwrap();
            gl.attach_shader(rendering_program_id, vert_shader);
            gl.attach_shader(rendering_program_id, frag_shader);
            gl.link_program(rendering_program_id);

            let position_loc = gl
                .get_attrib_location(rendering_program_id, "aPos")
                .unwrap();
            gl.enable_vertex_attrib_array(position_loc);

            let projection_matrix_loc = gl
                .get_uniform_location(rendering_program_id, "projection")
                .unwrap();
            let view_matrix_loc = gl
                .get_uniform_location(rendering_program_id, "view")
                .unwrap();
            let skybox_texture_loc = gl
                .get_uniform_location(rendering_program_id, "skybox")
                .unwrap();

            SkyboxShader { gl, rendering_program_id, position_loc, projection_matrix_loc, view_matrix_loc, skybox_texture_loc }
        }
    }

    pub fn use_shader(&self) {
        unsafe {
            self.gl.use_program(Some(self.rendering_program_id));
        }
    }

    pub fn set_attribute_buffers(&self, vertex_buffer_id: &NativeBuffer) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer_id.clone()));
            self.gl.vertex_attrib_pointer_f32(self.position_loc, 3, FLOAT, false, 3 * mem::size_of::<f32>() as i32, 0);
        }
    }

    pub fn set_projection_matrix(&self, matrix: &[f32]) {
        unsafe {
            self.gl
                .uniform_matrix_4_f32_slice(Some(&self.projection_matrix_loc), false, matrix);
        }
    }

    pub fn set_view_matrix(&self, matrix: &[f32]) {
        unsafe {
            self.gl
                .uniform_matrix_4_f32_slice(Some(&self.view_matrix_loc), false, matrix);
        }
    }

    pub fn set_skybox_texture_loc(&self, value: i32) {
        unsafe {
            self.gl
                .uniform_1_i32(Some(&self.skybox_texture_loc), value);
        }
    }
}