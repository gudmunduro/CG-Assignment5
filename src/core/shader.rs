use std::{fs, mem};

use glow::*;

use super::color::Color;

pub struct Shader3D<'a> {
    gl: &'a Context,
    rendering_program_id: NativeProgram,
    position_loc: u32,
    normal_loc: u32,
    uv_loc: u32,
    model_matrix_loc: NativeUniformLocation,
    projection_matrix_loc: NativeUniformLocation,
    view_matrix_loc: NativeUniformLocation,
    light_pos_loc: NativeUniformLocation,
    light_dif_loc: NativeUniformLocation,
    mat_dif_loc: NativeUniformLocation,
    eye_loc: NativeUniformLocation,
    light_ambient_loc: NativeUniformLocation,
    light_specular_loc: NativeUniformLocation,
    mat_spec_loc: NativeUniformLocation,
    mat_amb_loc: NativeUniformLocation,
    mat_shine_loc: NativeUniformLocation,
}

impl<'a> Shader3D<'a> {
    pub fn new(gl: &'a Context) -> Shader3D {
        unsafe {
            let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
            let shader_file = fs::read_to_string("./shaders/simple3D.vert").unwrap();
            gl.shader_source(vert_shader, &shader_file);
            gl.compile_shader(vert_shader);
            let res = gl.get_shader_compile_status(vert_shader);
            if !res {
                let info_log = gl.get_shader_info_log(vert_shader);
                panic!("Couldn't compile vertex shader\nShader compilation Log:\n{info_log}");
            }

            let frag_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
            let shader_file = fs::read_to_string("./shaders/simple3D.frag").unwrap();
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
                .get_attrib_location(rendering_program_id, "a_position")
                .unwrap();
            gl.enable_vertex_attrib_array(position_loc);

            let normal_loc = gl
                .get_attrib_location(rendering_program_id, "a_normal")
                .unwrap();
            gl.enable_vertex_attrib_array(normal_loc);

            let uv_loc = gl
                .get_attrib_location(rendering_program_id, "a_uv")
                .unwrap();
            gl.enable_vertex_attrib_array(uv_loc);

            let model_matrix_loc = gl
                .get_uniform_location(rendering_program_id, "u_model_matrix")
                .unwrap();
            let projection_matrix_loc = gl
                .get_uniform_location(rendering_program_id, "u_projection_matrix")
                .unwrap();
            let view_matrix_loc = gl
                .get_uniform_location(rendering_program_id, "u_view_matrix")
                .unwrap();
            let light_pos_loc = gl
                .get_uniform_location(rendering_program_id, "u_light_position")
                .unwrap();
            let light_dif_loc = gl
                .get_uniform_location(rendering_program_id, "u_light_diffuse")
                .unwrap();
            let mat_dif_loc = gl
                .get_uniform_location(rendering_program_id, "u_material_diffuse")
                .unwrap();
            let eye_loc = gl
                .get_uniform_location(rendering_program_id, "u_eye_position")
                .unwrap();
            let light_ambient_loc = gl
                .get_uniform_location(rendering_program_id, "u_light_ambient")
                .unwrap();
            let light_specular_loc = gl
                .get_uniform_location(rendering_program_id, "u_light_specular")
                .unwrap();
            let mat_spec_loc = gl
                .get_uniform_location(rendering_program_id, "u_material_specular")
                .unwrap();
            let mat_amb_loc = gl
                .get_uniform_location(rendering_program_id, "u_material_ambient")
                .unwrap();
            let mat_shine_loc = gl
                .get_uniform_location(rendering_program_id, "u_material_shininess")
                .unwrap();

            Shader3D {
                gl,
                rendering_program_id,
                position_loc,
                normal_loc,
                uv_loc,
                model_matrix_loc,
                projection_matrix_loc,
                view_matrix_loc,
                light_pos_loc,
                light_dif_loc,
                mat_dif_loc,
                eye_loc,
                light_ambient_loc,
                light_specular_loc,
                mat_spec_loc,
                mat_amb_loc,
                mat_shine_loc,
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.rendering_program_id));
        }
    }

    pub fn set_model_matrix(&self, matrix: &[f32]) {
        unsafe {
            self.gl
                .uniform_matrix_4_f32_slice(Some(&self.model_matrix_loc), false, matrix);
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

    pub fn set_position_attribute(&self, vertex_array: &[f32]) {
        unsafe {
            let array_ptr: u64 = mem::transmute(vertex_array.as_ptr());
            self.gl.vertex_attrib_pointer_f32(
                self.position_loc,
                3,
                FLOAT,
                false,
                0,
                array_ptr,
            );
        }
    }

    pub fn set_normal_attribute(&self, vertex_array: &[f32]) {
        unsafe {
            let array_ptr: u64 = mem::transmute(vertex_array.as_ptr());
            self.gl.vertex_attrib_pointer_f32(
                self.normal_loc,
                3,
                FLOAT,
                false,
                0,
                array_ptr,
            );
        }
    }

    pub fn set_uv_attribute(&self, uv_array: &[f32]) {
        unsafe {
            let array_ptr: u64 = mem::transmute(uv_array.as_ptr());
            self.gl.vertex_attrib_pointer_f32(
                self.uv_loc,
                2,
                FLOAT,
                false,
                0,
                array_ptr,
            );
        }
    }

    pub fn set_attribute_buffers(&self, vertex_buffer_id: &NativeBuffer) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer_id.clone()));
            self.gl.vertex_attrib_pointer_f32(self.position_loc, 3, FLOAT, false, 8 * mem::size_of::<f32>() as i32, 0);
            self.gl.vertex_attrib_pointer_f32(self.normal_loc, 3, FLOAT, false, 8 * mem::size_of::<f32>() as i32, 3 * mem::size_of::<f32>() as u64);
            self.gl.vertex_attrib_pointer_f32(self.uv_loc, 2, FLOAT, false, 8 * mem::size_of::<f32>() as i32, 2 * mem::size_of::<f32>() as u64);
        }
    }

    pub fn set_light_position(&self, light_positions: &[f32]) {
        unsafe {
            self.gl
                .uniform_4_f32_slice(Some(&self.light_pos_loc), light_positions);
        }
    }

    pub fn set_light_diffuse(&self, lights: &[f32]) {
        unsafe {
            self.gl
                .uniform_4_f32_slice(Some(&self.light_dif_loc), lights);
        }
    }

    pub fn set_eye_position(&self, x: f32, y: f32, z: f32) {
        unsafe {
            self.gl.uniform_4_f32(Some(&self.eye_loc), x, y, z, 1.0);
        }
    }

    pub fn set_light_ambient(&self, lights: &[f32]) {
        unsafe {
            self.gl
                .uniform_4_f32_slice(Some(&self.light_ambient_loc), lights);
        }
    }

    pub fn set_light_specular(&self, lights: &[f32]) {
        unsafe {
            self.gl
                .uniform_4_f32_slice(Some(&self.light_specular_loc), lights);
        }
    }

    pub fn set_material_diffuse(&self, color: &Color) {
        let Color { r, g, b } = color.clone();
        
        unsafe {
            self.gl.uniform_4_f32(Some(&self.mat_dif_loc), r, g, b, 0.0);
        }
    }

    pub fn set_material_specular(&self, color: &Color) {
        let Color { r, g, b } = color.clone();

        unsafe {
            self.gl.uniform_4_f32(Some(&self.mat_spec_loc), r, g, b, 0.0);
        }
    }

    pub fn set_material_ambient(&self, color: &Color) {
        let Color { r, g, b } = color.clone();

        unsafe {
            self.gl.uniform_4_f32(Some(&self.mat_amb_loc), r, g, b, 0.0);
        }
    }

    pub fn set_shininess(&self, s: f32) {
        unsafe {
            self.gl.uniform_1_f32(Some(&self.mat_shine_loc), s);
        }
    }
}
