use glow::Context;

use crate::{core::{game_object::GameObject, obj_loader::load_obj_file, game::Game}, objects::mesh_model::{MeshModel, self}};


pub struct BTest<'a> {
    mesh_model: MeshModel<'a>
}

impl<'a> BTest<'a> {
    pub fn new(gl: &'a Context) -> BTest {
        let mesh = load_obj_file("./models", "mouth.obj", gl).expect("Failed to load blender model");

        BTest {
            mesh_model: mesh
        }
    }
}

impl<'a> GameObject<'a> for BTest<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a glow::Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a glow::Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        model_matrix.add_translate(0.0, 0.0, -8.0);
        model_matrix.add_scale(5.0, 5.0, 5.0);

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.mesh_model.draw(&game.shader);
        model_matrix.pop_stack();
    }
}
