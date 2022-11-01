use std::collections::HashMap;

use itertools::Itertools;
use nalgebra::Vector3;

use super::{color::Color, shader::Shader3D};

struct Light {
    pub position: Vector3<f32>,
    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
    pub max_radius: f32,
}

impl Light {
    pub fn new() -> Light {
        Light {
            position: Vector3::zeros(),
            diffuse: Color::zeros(),
            ambient: Color::zeros(),
            specular: Color::zeros(),
            max_radius: 0.0,
        }
    }
}

pub struct Lights {
    // Value tuple is (start, end, radius_index) in lists
    lights: HashMap<String, Light>,
}

impl Lights {
    pub fn new() -> Lights {
        Lights {
            lights: HashMap::new(),
        }
    }

    pub fn add_light(&mut self, id: &str) {
        self.lights.insert(id.to_string(), Light::new());
    }

    pub fn remove_light(&mut self, id: &str) {
        self.lights.remove(id);
    }

    pub fn set_light_position(&mut self, id: &str, position: &Vector3<f32>) {
        let mut light = match self.lights.get_mut(id) {
            Some(l) => l,
            None => {
                log::error!("Tired to change position of invalid light {id}");
                return;
            }
        };

        light.position = position.clone();
    }

    pub fn set_light_diffuse(&mut self, id: &str, color: &Color) {
        let mut light = match self.lights.get_mut(id) {
            Some(l) => l,
            None => {
                log::error!("Tired to diffuse change color of invalid light {id}");
                return;
            }
        };

        light.diffuse = color.clone();
    }

    pub fn set_light_specular(&mut self, id: &str, color: &Color) {
        let mut light = match self.lights.get_mut(id) {
            Some(l) => l,
            None => {
                log::error!("Tired to specular change color of invalid light {id}");
                return;
            }
        };

        light.specular = color.clone();
    }

    pub fn set_light_ambient(&mut self, id: &str, color: &Color) {
        let mut light = match self.lights.get_mut(id) {
            Some(l) => l,
            None => {
                log::error!("Tired to ambient change color of invalid light {id}");
                return;
            }
        };

        light.ambient = color.clone();
    }

    pub fn set_light_max_radius(&mut self, id: &str, radius: f32) {
        let mut light = match self.lights.get_mut(id) {
            Some(l) => l,
            None => {
                log::error!("Tired to max radius of invalid light {id}");
                return;
            }
        };

        light.max_radius = radius;
    }

    pub fn update_lights(&self, shader: &Shader3D) {
        shader.set_light_count(self.lights.len() as u32);
        shader.set_light_position(
            &self
                .lights
                .values()
                .flat_map(|l| [l.position.x, l.position.y, l.position.z, 1.0])
                .collect_vec(),
        );
        shader.set_light_diffuse(
            &self
                .lights
                .values()
                .flat_map(|l| [l.diffuse.r, l.diffuse.g, l.diffuse.b, l.diffuse.a])
                .collect_vec(),
        );
        shader.set_light_ambient(
            &self
                .lights
                .values()
                .flat_map(|l| [l.ambient.r, l.ambient.g, l.ambient.b, l.ambient.a])
                .collect_vec(),
        );
        shader.set_light_specular(
            &self
                .lights
                .values()
                .flat_map(|l| [l.specular.r, l.specular.g, l.specular.b, l.specular.a])
                .collect_vec(),
        );
        shader.set_light_max_radius(&self.lights.values().map(|l| l.max_radius).collect_vec());
    }
}
