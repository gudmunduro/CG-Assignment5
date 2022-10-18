use glow::Context;

use crate::{core::{game_object::GameObject, obj_loader::load_obj_file}, objects::mesh_model::{MeshModel, self}};


pub struct BTest<'a> {
    mesh_model: MeshModel<'a>
}

impl<'a> GameObject<'a> for BTest<'a> {
    fn update(&mut self, game: &crate::core::game::Game, gl: &'a glow::Context) {
        
    }

    fn display(&self, game: &crate::core::game::Game, gl: &'a glow::Context) {
        self.mesh_model.draw(&game.shader);
    }
}

impl<'a> BTest<'a> {
    pub fn new(gl: &'a Context) -> BTest {
        let mesh = load_obj_file("./models", "mouth.obj", gl).unwrap();

        BTest {
            mesh_model: mesh
        }
    }
}