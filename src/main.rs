pub mod core;
pub mod objects;
pub mod game_objects;
pub mod utils;

use crate::core::constants::{W_HEIGHT, W_WIDTH};

use crate::core::game;
use crate::core::matrices;
use crate::core::shader::Shader3D;
use crate::objects::cube::Cube;
use glow::*;

fn main() {
    let (gl, window, mut events_loop, gl_context) = unsafe {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        let window = video
            .window("OpenGL", W_WIDTH, W_HEIGHT)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let events_loop = sdl.event_pump().unwrap();

        (gl, window, events_loop, gl_context)
    };

    let mut game = game::Game::new(
        &gl,
        &window,
        &mut events_loop,
        &gl_context,
    );
    game.create_scene();

    game.main();
}
