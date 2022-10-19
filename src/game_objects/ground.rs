use glow::Context;

use crate::core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color};


pub struct Ground {
}

impl Ground {
    pub fn new() -> Ground {

        Ground {}
    }
}

impl<'a> GameObject<'a> for Ground {

    fn collision_info(&self) -> CollisionInfo {
        CollisionInfo::YCollision(-0.2)
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(0.6);
        game.shader.set_material_diffuse(&Color::new(0.3, 0.14, 0.08));
        game.shader.set_material_specular(&Color::new(0.3, 0.14, 0.08));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();
        model_matrix.add_translate(0.0, -0.2, 0.0);
        model_matrix.add_scale(350.0, 0.2, 350.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        game.cube.draw(&game.shader);

        model_matrix.pop_stack();
    }
}
