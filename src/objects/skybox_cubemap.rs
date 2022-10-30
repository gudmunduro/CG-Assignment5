use std::slice;

use glow::*;
use itertools::{Itertools, izip};

use crate::core::{shader::Shader3D, skybox_shader::SkyboxShader};


pub struct SkyboxCubemap<'a> {
    buffer: NativeBuffer,
    gl: &'a Context
}

impl<'a> SkyboxCubemap<'a> {
    pub fn new(gl: &'a Context) -> SkyboxCubemap {
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
            // Back
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,

            // Front
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,

            // Bottom
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,

            // Top
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,

            // Right
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,

            // Left
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
        ];

        let vertex_array = izip!(
            position_array.iter().tuples(),
            normal_array.iter().tuples(),
            uv_array.iter().tuples()
        )
        .flat_map(|((&px, &py, &pz), (&nx, &ny, &nz), (&u, &v))| [px, py, pz, nx, ny, nz, u, v])
        .collect::<Vec<f32>>();
        
        let buffer = unsafe {
            let buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(ARRAY_BUFFER, Some(buffer));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, slice::from_raw_parts(vertex_array.as_ptr() as *const u8, vertex_array.len() * 4), STATIC_DRAW);
            gl.bind_buffer(ARRAY_BUFFER, None);
            buffer
        };

        SkyboxCubemap { buffer, gl }
    }

    pub fn draw(&self, shader: &Shader3D, textures: &Vec<NativeTexture>) {
        shader.set_attribute_buffers(&self.buffer);
        shader.set_diffuse_texture_active(true);
        shader.set_specular_texture_active(false);

        unsafe {
            self.gl.active_texture(TEXTURE0);
            shader.set_diffuse_texture(0);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[0].clone()));

            self.gl.draw_arrays(TRIANGLE_FAN, 0, 4);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[1].clone()));
            self.gl.draw_arrays(TRIANGLE_FAN, 4, 4);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[2].clone()));
            self.gl.draw_arrays(TRIANGLE_FAN, 8, 4);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[3].clone()));
            self.gl.draw_arrays(TRIANGLE_FAN, 12, 4);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[4].clone()));
            self.gl.draw_arrays(TRIANGLE_FAN, 16, 4);
            self.gl.bind_texture(TEXTURE_2D, Some(textures[5].clone()));
            self.gl.draw_arrays(TRIANGLE_FAN, 20, 4);
        }
    }
}