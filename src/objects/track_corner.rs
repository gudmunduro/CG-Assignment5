use std::slice;

use glow::*;
use itertools::{izip, Itertools};
use nalgebra::{Vector2, Vector3};

use crate::core::shader::Shader3D;

const LINE_ACCURACY: i32 = 50;

pub struct TrackCorner<'a> {
    buffer: NativeBuffer,
    gl: &'a Context
}

impl<'a> TrackCorner<'a> {
    pub fn new(gl: &'a Context, enter: Vector3<f32>, control: Vector3<f32>, exit: Vector3<f32>, track_width: f32) -> TrackCorner {
        let pb = 0.5 * enter + 0.5 * exit;

        let mut position_array = Vec::with_capacity((LINE_ACCURACY * 6) as usize);
        let mut normal_array = Vec::with_capacity((LINE_ACCURACY * 6) as usize);
        for i in 0..LINE_ACCURACY {
            let t = i as f32 / LINE_ACCURACY as f32;
            let track_outer = TrackCorner::bezier_curve(&enter, &control, &exit, t);

            let inner_direction = (pb - track_outer).normalize();
            let track_inner = track_outer + inner_direction * track_width;

            position_array.extend(vec![track_outer.x, track_outer.y, track_outer.z, track_inner.x, track_inner.y, track_inner.z]);
            normal_array.extend(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0]);
        }

        let uv_array: Vec<f32> = (0..LINE_ACCURACY * 2)
            .flat_map(|i| match i % 4 {
                0 => [1.0, 1.0],
                1 => [1.0, 0.0],
                2 => [0.0, 1.0],
                3 => [0.0, 0.0],
                _ => [0.0, 0.0]
            })
            .collect();

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

        TrackCorner { buffer, gl }
    }

    pub fn bezier_curve(p0: &Vector3<f32>, p1: &Vector3<f32>, p2: &Vector3<f32>, t: f32) -> Vector3<f32> {
        (1.0 - t).powi(2) * p0 + 2.0 * (1.0-t) * t * p1 + t.powi(2) * p2
    }

    pub fn draw(&self, shader: &Shader3D, texture: &NativeTexture) {
        shader.set_attribute_buffers(&self.buffer);
        shader.set_render_texture(true);
        
        unsafe {
            self.gl.bind_texture(TEXTURE_2D, Some(*texture));

            self.gl.draw_arrays(TRIANGLE_STRIP, 0, LINE_ACCURACY * 2);
        }
    }
}