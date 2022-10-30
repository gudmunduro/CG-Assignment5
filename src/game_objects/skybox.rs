use glow::*;

use crate::{core::{game::Game, game_object::GameObject}, objects::skybox_cubemap::SkyboxCubemap};


pub struct Skybox<'a> {
    skybox_texture: NativeTexture,
    skybox_cubemap: SkyboxCubemap<'a>,
}

impl<'a> Skybox<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Skybox<'a> {
        let skybox_faces = vec![
            "./models/textures/skybox/right.jpg",
            "./models/textures/skybox/left.jpg",
            "./models/textures/skybox/top.jpg",
            "./models/textures/skybox/bottom.jpg",
            "./models/textures/skybox/front.jpg",
            "./models/textures/skybox/back.jpg",
        ];
        let skybox_texture = game.load_cubemap(&skybox_faces);
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

        game.skybox_shader.use_shader();
        game.skybox_shader.set_view_matrix(game.view_matrix.borrow().get_matrix().as_slice());
        game.skybox_shader.set_projection_matrix(game.projection_matrix.borrow().get_matrix().as_slice());
        game.skybox_shader.set_skybox_texture_loc(0);

        self.skybox_cubemap.draw(&game.skybox_shader, &self.skybox_texture);
        
        unsafe {
            gl.depth_mask(false);
        }

        // Switch back to default shader so other objects are drawn correctly
        game.shader.use_shader();
    }
}

