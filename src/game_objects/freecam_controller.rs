use glow::*;
use nalgebra::Vector3;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    core::{
        game::Game,
        game_object::GameObject,
        obj_loader::{load_mtl_file, load_obj_file},
    },
    objects::mesh_model::MeshModel,
};

enum ArrowDir {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct FreecamController {
    moving_foward: bool,
    moving_backward: bool,
    arrow_direction: ArrowDir,
}

impl<'a> FreecamController {
    pub fn new(gl: &Context) -> FreecamController {
        FreecamController {
            moving_foward: false,
            moving_backward: false,
            arrow_direction: ArrowDir::None,
        }
    }
}

impl<'a> GameObject<'a> for FreecamController {
    fn on_event(&mut self, game: &Game, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                self.moving_foward = true;
            },
            Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                self.moving_foward = false;
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.moving_backward = true;
            },
            Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                self.moving_backward = false;
            },
            Event::KeyDown { keycode: Some(code @ (Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right)), .. } => {
                use ArrowDir::*;
                self.arrow_direction = match code {
                    Keycode::Up => Up,
                    Keycode::Down => Down,
                    Keycode::Left => Left,
                    Keycode::Right => Right,
                    _ => None
                };
            },
            Event::KeyUp { keycode: Some(Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right), .. } => {
                self.arrow_direction = ArrowDir::None;
            },
            _ => (),
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        let mut view_matrix = game.view_matrix.borrow_mut();

        if self.moving_foward {
            let n = view_matrix.n;
            view_matrix.slide(0.0, 0.0, -game.delta_time * 10.0, Vector3::zeros(), Vector3::zeros(), n);
        }

        if self.moving_backward {
            let n = view_matrix.n;
            view_matrix.slide(0.0, 0.0, game.delta_time  * 10.0, Vector3::zeros(), Vector3::zeros(), n);
        }        

        let rot_speed = 150.0;
        use ArrowDir::*;
        match self.arrow_direction {
            Up => view_matrix.pitch(rot_speed * game.delta_time),
            Down => view_matrix.pitch(-rot_speed * game.delta_time),
            Left => view_matrix.yaw(rot_speed * game.delta_time),
            Right => view_matrix.yaw(-rot_speed * game.delta_time),
            None => ()
        }
    }

    fn display(&self, game: &Game, gl: &'a Context) {}
}
