use glow::{Context, NativeTexture};
use nalgebra::{Vector3, Vector2};

use crate::{core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color}, objects::textured_cube::TexturedCube};


pub struct RoadSurface<'a> {
    pos: Vector3<f32>,
    size: Vector2<f32>,
    surface_cube: TexturedCube<'a>,
    road_texture: NativeTexture
}

impl<'a> RoadSurface<'a> {
    pub fn new(pos: Vector3<f32>, size: Vector2<f32>, gl: &'a Context, game: &Game) -> RoadSurface<'a> {
        let road_texture = game.load_texture("./models/textures/road.png");
        let surface_cube = TexturedCube::new(gl);

        RoadSurface { pos, size, surface_cube, road_texture }
    }
}

impl<'a> GameObject<'a> for RoadSurface<'a> {

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(&Color::new(0.3, 0.14, 0.08));
        game.shader.set_material_diffuse(&Color::new(0.3, 0.14, 0.08));
        game.shader.set_material_specular(&Color::new(0.3, 0.14, 0.08));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();
        model_matrix.add_translate(self.pos.x, self.pos.y, self.pos.z);
        model_matrix.add_scale(self.size.x, 0.2, self.size.y);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.surface_cube.draw(&game.shader, &self.road_texture);

        model_matrix.pop_stack();
    }
}
