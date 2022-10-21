use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        ambient: Option<Color>,
        diffuse: Option<Color>,
        specular: Option<Color>,
        shininess: Option<f32>,
    ) -> Material {
        Material {
            ambient: ambient.unwrap_or(Color::zeros()),
            diffuse: diffuse.unwrap_or(Color::zeros()),
            specular: specular.unwrap_or(Color::zeros()),
            shininess: shininess.unwrap_or(0.0),
        }
    }
}
