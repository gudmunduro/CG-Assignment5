pub mod core;
pub mod game_objects;
pub mod network;
pub mod objects;
pub mod utils;

use clap::Parser;
use simplelog::TermLogger;

use crate::core::constants::{W_HEIGHT, W_WIDTH};

use crate::core::game;

/// Assignment 5 game
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Server ip and port in the form IP:PORT.
    /// Skip this argument to play in single player
    #[clap(short, long, default_value = None)]
    server: Option<String>,
}

fn main() {
    let args = Args::parse();
    init_logger();

    let (gl, window, mut events_loop, _gl_context, joystick) = unsafe {
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

        let joystick = sdl.joystick().unwrap();

        (gl, window, events_loop, gl_context, joystick)
    };

    let mut game = game::Game::new(
        &gl,
        &window,
        &mut events_loop,
        &joystick,
        args.server.as_ref().map(String::as_str),
    );
    game.create_scene();

    game.main();
}

fn init_logger() {
    TermLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Failed to init logger");
}
