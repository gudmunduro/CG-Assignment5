use glow::{Context, NativeTexture, HasContext, TEXTURE_2D};

use crate::{core::{game_object::GameObject, obj_loader::load_obj_file, game::Game, color::Color}, objects::{mesh_model::{MeshModel, self}, dice::Dice}};


pub struct BTest<'a> {
    dice: Dice<'a>,
    texture: NativeTexture,
}

impl<'a> BTest<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> BTest<'a> {
        // let mesh = load_obj_file("./models", "car.obj", gl, game).expect("Failed to load blender model");
        let texture = game.load_texture("./models/textures/dice.png");

        BTest {
            dice: Dice::new(gl),
            texture,
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
        model_matrix.add_translate(0.0, 2.0, -8.0);
        model_matrix.add_scale(2.0, 2.0, 2.0);

        game.shader.set_material_ambient(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_material_diffuse(&Color::new(0.0, 0.0, 0.0));
        game.shader.set_material_specular(&Color::new(0.0, 0.0, 0.0));
        game.shader.set_shininess(3.0);

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }

        self.dice.draw(&game.shader);
        model_matrix.pop_stack();
    }
}
