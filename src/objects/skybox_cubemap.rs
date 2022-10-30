use std::slice;

use glow::*;

use crate::core::{shader::Shader3D, skybox_shader::SkyboxShader};


pub struct SkyboxCubemap<'a> {
    buffer: NativeBuffer,
    gl: &'a Context
}

impl<'a> SkyboxCubemap<'a> {
    pub fn new(gl: &'a Context) -> SkyboxCubemap {
        let vertices = [
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0
        ];

        let buffer = unsafe {
            let buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(ARRAY_BUFFER, Some(buffer));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * 4), STATIC_DRAW);
            gl.bind_buffer(ARRAY_BUFFER, None);
            buffer
        };

        SkyboxCubemap { buffer, gl }
    }

    pub fn draw(&self, shader: &SkyboxShader, texture: &NativeTexture) {
        shader.set_attribute_buffers(&self.buffer);
        
        unsafe {
            self.gl.active_texture(TEXTURE0);
            self.gl.bind_texture(TEXTURE_CUBE_MAP, Some(*texture));
            self.gl.draw_arrays(TRIANGLES, 0, 36);
        }
    }
}