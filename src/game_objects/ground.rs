use glow::{Context, NativeTexture};

use crate::{core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color}, objects::textured_square::TexturedSquare, utils::FacingDirection};


pub struct Ground<'a> {
    texture: NativeTexture,
    ground: TexturedSquare<'a>
}

impl<'a> Ground<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Ground<'a> {
        let texture = game.load_texture("./models/textures/desert.png", true);
        let ground = TexturedSquare::new(gl, 800.0, 800.0, FacingDirection::North);

        Ground { ground, texture }
    }
}

impl<'a> GameObject<'a> for Ground<'a> {

    fn collision_info(&self) -> CollisionInfo {
        CollisionInfo::YCollision(-0.2)
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(&Color::new(0.5, 0.5, 0.5));
        game.shader.set_material_diffuse(&Color::new(8.0, 0.8, 0.8));
        game.shader.set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();
        model_matrix.add_translate(0.0, -0.2, 0.0);
        // model_matrix.add_scale(350.0, 0.2, 350.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.ground.draw(&game.shader, &self.texture);

        model_matrix.pop_stack();
    }
}
