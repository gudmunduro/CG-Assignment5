use std::num::NonZeroU32;

use glow::*;

use crate::core::shader::Shader3D;

pub struct Dice<'a> {
    position_array: [f32; 72],
    normal_array: [f32; 72],
    uv_array: [f32; 48],
    gl: &'a Context
}

impl<'a> Dice<'a> {
    pub fn new(gl: &'a Context) -> Dice {
        let position_array = [-0.5, -0.5, -0.5,
        -0.5, 0.5, -0.5,
        0.5, 0.5, -0.5,
        0.5, -0.5, -0.5,
        -0.5, -0.5, 0.5,
        -0.5, 0.5, 0.5,
        0.5, 0.5, 0.5,
        0.5, -0.5, 0.5,
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, 0.5,
        -0.5, -0.5, 0.5,
        -0.5, 0.5, -0.5,
        0.5, 0.5, -0.5,
        0.5, 0.5, 0.5,
        -0.5, 0.5, 0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, 0.5,
        -0.5, 0.5, 0.5,
        -0.5, 0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5, 0.5,
        0.5, 0.5, 0.5,
        0.5, 0.5, -0.5];

        let normal_array = [0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, -1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0];
        let uv_array = [
            0.33, 1.0, 0.33, 0.66, 0.66, 1.0, 0.66, 0.66, // 3
            0.66, 1.0, 0.66, 0.66, 1.0, 1.0, 1.0, 0.6, // 6
            0.0, 0.66, 0.0, 0.33, 0.33, 0.66, 0.33, 0.33, // 2
            0.33, 0.66, 0.33, 0.33, 0.66, 0.66, 0.66, 0.33, // 1
            0.66, 0.66, 0.66, 0.33, 1.0, 0.66, 1.0, 0.33, // 5
            0.33, 0.33, 0.33, 0.0, 0.66, 0.33, 0.66, 0.0, // 4 
        ];

        Dice { position_array, normal_array, uv_array, gl }
    }

    pub fn draw(&self, shader: &Shader3D) {
        shader.set_position_attribute(&self.position_array);
        shader.set_normal_attribute(&self.normal_array);
        shader.set_uv_attribute(&self.uv_array);
        
        unsafe {
            self.gl.draw_arrays(TRIANGLE_FAN, 0, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 4, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 8, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 12, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 16, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 20, 4);
        }
    }
}