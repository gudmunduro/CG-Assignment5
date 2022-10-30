use glow::*;

use crate::{core::{game::Game, game_object::GameObject}, objects::skybox_cubemap::SkyboxCubemap};


pub struct Skybox<'a> {
    skybox_texture: Vec<NativeTexture>,
    skybox_cubemap: SkyboxCubemap<'a>,
}

impl<'a> Skybox<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Skybox<'a> {
        let skybox_faces = vec![
            "./models/textures/skybox2/front.png",
            "./models/textures/skybox2/back.png",
            "./models/textures/skybox2/down.png",
            "./models/textures/skybox2/up.png",
            "./models/textures/skybox2/left.png",
            "./models/textures/skybox2/right.png",
        ];
        let skybox_texture = skybox_faces.iter().map(|t| game.load_texture(*t, false)).collect();
        let skybox_cubemap = SkyboxCubemap::new(gl);

        Skybox { skybox_texture, skybox_cubemap }
    }
}

impl<'a> GameObject<'a> for Skybox<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        unsafe {
            gl.depth_mask(false);
        }

        let mut model_matrix = game.model_matrix.borrow_mut();
        model_matrix.push_stack();
        model_matrix.add_translate(0.0, 0.0, 0.0);
        model_matrix.add_scale(400.0, 400.0, 400.0);
        
        game.shader.set_view_matrix(game.view_matrix.borrow().get_matrix_no_tranlate().as_slice());
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        game.shader.set_skybox_mode(true);
        self.skybox_cubemap.draw(&game.shader, &self.skybox_texture);
        game.shader.set_view_matrix(game.view_matrix.borrow().get_matrix().as_slice());
        game.shader.set_skybox_mode(false);
        
        model_matrix.pop_stack();
        unsafe {
            gl.depth_mask(true);
        }
    }
}

