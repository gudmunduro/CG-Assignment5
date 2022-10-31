use glow::Context;
use nalgebra::Vector3;

use crate::{core::{game_object::GameObject, obj_loader::load_obj_file, constants::MODEL_LOCATION, game::Game}, objects::mesh_model::MeshModel};

pub enum CactusType {
    Small,
    Large,
}

pub struct Cactus<'a> {
    position: Vector3<f32>,
    rotation: f32,
    model: MeshModel<'a>,
}

impl<'a> Cactus<'a> {
    pub fn new(position: Vector3<f32>, rotation: f32, cactus_type: CactusType, gl: &'a Context, game: &Game) -> Cactus<'a> {
        use CactusType::*;
        let model_file = match cactus_type {
            Small => "cactus-small.obj",
            Large => "cactus-large.obj"
        };

        let model = load_obj_file(MODEL_LOCATION, model_file, gl, game)
            .expect("Failed to load model");

        Cactus { position, rotation, model }
    }
}

impl<'a> GameObject<'a> for Cactus<'a> {
    fn on_event(&mut self, _game: &crate::core::game::Game, _event: &sdl2::event::Event) {}

    fn update(&mut self, _game: &crate::core::game::Game, _gl: &'a glow::Context) {}

    fn display(&self, game: &crate::core::game::Game, _gl: &'a glow::Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        
        model_matrix.add_translate(self.position.x, self.position.y, self.position.z);
        model_matrix.add_scale(3.0, 3.0, 3.0);
        model_matrix.add_rotation(0.0, self.rotation, 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.model.draw(&game.shader);

        model_matrix.pop_stack();
    }
}
