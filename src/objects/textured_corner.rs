use glow::*;
use nalgebra::Vector2;

use crate::core::shader::Shader3D;

const LINE_ACCURACY: i32 = 50;

pub struct TexturedCorner<'a> {
    position_array: Vec<f32>,
    normal_array: Vec<f32>,
    uv_array: [f32; 48],
    gl: &'a Context
}

impl<'a> TexturedCorner<'a> {
    pub fn new(gl: &'a Context) -> TexturedCorner {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(50.0, 100.0);
        let p2 = Vector2::new(100.0, 0.0);
        let pb = 0.5 * p2 + 0.5 * p0;

        let mut position_array = Vec::with_capacity((LINE_ACCURACY * 6) as usize);
        let mut normal_array = Vec::with_capacity((LINE_ACCURACY * 6) as usize);
        for i in 0..LINE_ACCURACY {
            let t = i as f32 / LINE_ACCURACY as f32;
            let pos = TexturedCorner::bezier_interpolate(&p0, &p1, &p2, t);
            let pos2 = (0.5) * pos + (0.5) * pb;

            position_array.extend(vec![pos.x, 0.0, pos.y, pos2.x, 0.0, pos2.y]);
            normal_array.extend(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0]);
        }

        let uv_array = [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ];

        TexturedCorner { position_array, normal_array, uv_array, gl }
    }

    pub fn bezier_interpolate(p0: &Vector2<f32>, p1: &Vector2<f32>, p2: &Vector2<f32>, t: f32) -> Vector2<f32> {
        (1.0 - t).powi(2) * p0 + 2.0 * (1.0-t) * t * p1 + t.powi(2) * p2
    }

    pub fn draw(&self, shader: &Shader3D, texture: &NativeTexture) {
        shader.set_position_attribute(&self.position_array);
        shader.set_normal_attribute(&self.normal_array);
        shader.set_render_texture(false);
        
        unsafe {
            self.gl.bind_texture(TEXTURE_2D, None);

            self.gl.draw_arrays(TRIANGLE_STRIP, 0, LINE_ACCURACY * 2);
        }
    }
}