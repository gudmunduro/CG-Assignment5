use std::collections::HashMap;

use nalgebra::Vector3;

use super::{color::Color, shader::Shader3D};

pub struct Lights {
    positions: Vec<f32>,
    diffuse: Vec<f32>,
    ambient: Vec<f32>,
    specular: Vec<f32>,
    radius: Vec<f32>,
    // Value tuple is (start, end, radius_index) in lists
    lights: HashMap<String, (usize, usize, usize)>,
}

impl Lights {
    pub fn new() -> Lights {
        Lights {
            positions: Vec::new(),
            diffuse: Vec::new(),
            ambient: Vec::new(),
            specular: Vec::new(),
            radius: Vec::new(),
            lights: HashMap::new(),
        }
    }

    pub fn add_light(&mut self, id: &str) {
        let start = self.lights.len();
        let end = self.lights.len() + 4;
        let radius_index = self.radius.len();

        self.lights.insert(id.to_string(), (start, end, radius_index));
        self.positions.extend([0.0, 0.0, 0.0, 0.0]);
        self.diffuse.extend([0.0, 0.0, 0.0, 0.0]);
        self.ambient.extend([0.0, 0.0, 0.0, 0.0]);
        self.specular.extend([0.0, 0.0, 0.0, 0.0]);
        self.radius.push(0.0);
    }

    pub fn set_light_position(&mut self, id: &str, position: &Vector3<f32>) {
        let (start, end, _) = match self.lights.get(id) {
            Some(v) => *v,
            None => {
                log::error!("Tired to change position of invalid light {id}");
                return;
            },
        };

        self
            .positions
            .splice(start..end, [position.x, position.y, position.z, 1.0]);
    }

    pub fn set_light_diffuse(&mut self, id: &str, color: &Color) {
        let (start, end, _) = match self.lights.get(id) {
            Some(v) => *v,
            None => {
                log::error!("Tired to diffuse change color of invalid light {id}");
                return;
            },
        };

        self
            .diffuse
            .splice(start..end, [color.r, color.g, color.b, color.a]);
    }

    pub fn set_light_specular(&mut self, id: &str, color: &Color) {
        let (start, end, _) = match self.lights.get(id) {
            Some(v) => *v,
            None => {
                log::error!("Tired to specular change color of invalid light {id}");
                return;
            },
        };

        self
            .specular
            .splice(start..end, [color.r, color.g, color.b, color.a]);
    }

    pub fn set_light_ambient(&mut self, id: &str, color: &Color) {
        let (start, end, _) = match self.lights.get(id) {
            Some(v) => *v,
            None => {
                log::error!("Tired to ambient change color of invalid light {id}");
                return;
            },
        };

        self
            .ambient
            .splice(start..end, [color.r, color.g, color.b, color.a]);
    }

    pub fn set_light_max_radius(&mut self, id: &str, radius: f32) {
        let (_, _, radius_index) = match self.lights.get(id) {
            Some(v) => *v,
            None => {
                log::error!("Tired to max radius of invalid light {id}");
                return;
            },
        };

        self.radius[radius_index] = radius;
    }

    pub fn update_lights(&self, shader: &Shader3D) {
        shader.set_light_count(self.lights.len() as u32);
        shader.set_light_position(&self.positions);
        shader.set_light_diffuse(&self.diffuse);
        shader.set_light_ambient(&self.ambient);
        shader.set_light_specular(&self.specular);
        shader.set_light_max_radius(&self.radius);
    }
}
