use std::f32;

use glow::Context;
use nalgebra::Vector3;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{core::{game_object::GameObject, game::Game}, game_objects::car::ViewState};

use super::car::Car;

const LOOK_DIST: f32 = 0.9;

pub struct PlayerCar<'a> {
    car: Car<'a>
}

impl<'a> PlayerCar<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> PlayerCar<'a> {
        let car = Car::new(true, gl, game);

        PlayerCar { car }
    }
}

impl<'a> GameObject<'a> for PlayerCar<'a> {
    fn on_event(&mut self, game: &Game, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key ),
                ..
            } => {
                use Keycode::*;
                match key {
                    W => {
                        self.car.set_throttle(100.0);
                    }
                    S => {
                        self.car.set_throttle(0.0);
                        self.car.set_brake(100.0);
                    }
                    A => {
                        self.car.set_steering_angle((f32::consts::PI / 4.0) * 0.25);
                    }
                    D => {
                        self.car.set_steering_angle((-std::f32::consts::PI / 4.0) * 0.25)
                    }
                    Space => {
                        self.car.set_handbrake(true);
                    }
                    V => {
                        let view_state = match self.car.view_state() {
                            ViewState::ThirdPerson => ViewState::FirstPerson,
                            ViewState::FirstPerson => ViewState::ThirdPerson
                        };
                        self.car.set_view_state(view_state);
                    }
                    _ => ()
                }
            }
            Event::KeyUp { keycode: Some(key), .. }  => {
                use Keycode::*;
                match key {
                    W => {
                        self.car.set_throttle(0.0);
                    }
                    S => {
                        self.car.set_brake(0.0);
                    }
                    A | D => {
                        self.car.set_steering_angle(0.0);
                    }
                    Space => {
                        self.car.set_handbrake(false);
                    }
                    // TODO: Remove this after testing
                    L => {
                        self.car.set_y_velocity(20.0);
                    }
                    _ => (),
                }
            }
            _ => (),
        }   
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        self.car.update(game, gl);

        // Send status update
        if game.server_connection.is_multiplayer() {
            game.server_connection.send_status_update(self.car.position(), self.car.angle(), self.car.steering_angle());
        }

        let mut view_matrix = game.view_matrix.borrow_mut();
        let ang_sin = self.car.angle().sin();
        let ang_cos = self.car.angle().cos();

        // Update camera pos
        use ViewState::*;
        let eye = match self.car.view_state() {
            FirstPerson => {
                self.car.position() + Vector3::new(ang_sin * -1.8, 1.2, ang_cos * -1.8)
            }
            ThirdPerson => {
                self.car.position() + Vector3::new(ang_sin * -20.0, 6.0, ang_cos * -20.0)
            }
        };
        let center = eye + Vector3::new(ang_sin * LOOK_DIST, 0.0, ang_cos * LOOK_DIST);
        view_matrix.look(eye, center, Vector3::new(0.0, 1.0, 0.0));
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        self.car.display(game, gl);
    }
}