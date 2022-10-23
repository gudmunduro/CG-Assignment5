use std::slice;

use glow::*;
use itertools::{izip, Itertools};

use crate::{core::shader::Shader3D, utils::FacingDirection};

const MAX_SIZE: f32 = 20.0;

pub struct TexturedSquare<'a> {
    buffer: NativeBuffer,
    vertex_count: usize,
    gl: &'a Context,
}

impl<'a> TexturedSquare<'a> {
    pub fn new(gl: &'a Context, width: f32, height: f32, texture_dir: FacingDirection) -> TexturedSquare {
        let width_segment_count = (width as i32 / MAX_SIZE as i32).max(2);
        let height_segment_count = (height as i32 / MAX_SIZE as i32).max(2);
        let width_remaining_size = width % MAX_SIZE;
        let height_remaining_size = height % MAX_SIZE;
        let x_start = -width / 2.0;
        let y_start = -height / 2.0;

        // Split the square into segments, no larger than MAX_SIZE, to make lighting and (repeating) textures work better
        let position_array: Vec<f32> = (0..width_segment_count)
            .tuple_windows()
            .flat_map(|(i, i2)| {
                (0..height_segment_count)
                    .tuple_windows()
                    .flat_map(move |(j, j2)| {
                        let x_min = x_start + i as f32 * MAX_SIZE;
                        let y_min = y_start + j as f32 * MAX_SIZE;

                        let x_max = if i2 == width_segment_count - 1 && width_remaining_size != 0.0
                        {
                            x_start + i as f32 * MAX_SIZE + width_remaining_size
                        } else {
                            x_start + i2 as f32 * MAX_SIZE
                        };

                        let y_max = if j2 == height_segment_count && height_remaining_size != 0.0 {
                            y_start + j as f32 * MAX_SIZE + height_remaining_size
                        } else {
                            y_start + j2 as f32 * MAX_SIZE
                        };

                        [
                            x_min,
                            0.0,
                            y_min, // Down left
                            x_max,
                            0.0,
                            y_min, // Down right
                            x_max,
                            0.0,
                            y_max + 0.05, // Up right
                            x_min,
                            0.0,
                            y_max + 0.05, // Up left
                        ]
                    })
            })
            .collect();

        let normal_array: Vec<f32> = (0..position_array.len() / 3)
            .flat_map(|_| [0.0, 1.0, 0.0])
            .collect();

        use FacingDirection::*;
        let uv_array: Vec<f32> = match texture_dir {
            North => {
                (0..position_array.len() / 3)
                    .flat_map(|i| match i % 4 {
                        0 => [0.0, 0.0],
                        1 => [0.0, 1.0],
                        2 => [1.0, 1.0],
                        3 => [1.0, 0.0],
                        _ => [0.0, 0.0],
                    })
                    .collect()
            }
            West => {
                (0..position_array.len() / 3)
                    .flat_map(|i| match i % 4 {
                        0 => [0.0, 0.0],
                        1 => [1.0, 0.0],
                        2 => [1.0, 1.0],
                        3 => [0.0, 1.0],
                        _ => [0.0, 0.0],
                    })
                    .collect()
            }
        };

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

        TexturedSquare {
            buffer,
            vertex_count: position_array.len() / 3,
            gl,
        }
    }

    pub fn draw(&self, shader: &Shader3D, texture: &NativeTexture) {
        shader.set_attribute_buffers(&self.buffer);
        shader.set_render_texture(true);

        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(texture.clone()));

            for i in (0..self.vertex_count).step_by(4) {
                self.gl.draw_arrays(TRIANGLE_FAN, i as i32, 4);
            }
        }
    }
}
