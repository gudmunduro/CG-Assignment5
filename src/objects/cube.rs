use std::slice;

use glow::*;
use itertools::Itertools;

use crate::core::shader::Shader3D;

pub struct Cube<'a> {
    buffer: NativeBuffer,
    gl: &'a Context
}

impl<'a> Cube<'a> {
    pub fn new(gl: &'a Context) -> Cube {
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

        let vertex_array = position_array.iter()
        .tuples()
        .zip(normal_array.iter().tuples())
        .flat_map(|((&px, &py, &pz), (&nx, &ny, &nz))| [px, py, pz, nx, ny, nz, 0.0, 0.0])
        .collect::<Vec<f32>>();

        let buffer = unsafe {
            let buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(ARRAY_BUFFER, Some(buffer));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, slice::from_raw_parts(vertex_array.as_ptr() as *const u8, vertex_array.len() * 4), STATIC_DRAW);
            gl.bind_buffer(ARRAY_BUFFER, None);
            buffer
        };

        Cube { buffer, gl }
    }

    pub fn draw(&self, shader: &Shader3D) {
        shader.set_attribute_buffers(&self.buffer);
        shader.set_render_texture(false);
        
        unsafe {
            self.gl.bind_texture(TEXTURE_2D, None);

            self.gl.draw_arrays(TRIANGLE_FAN, 0, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 4, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 8, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 12, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 16, 4);
            self.gl.draw_arrays(TRIANGLE_FAN, 20, 4);
        }
    }
}