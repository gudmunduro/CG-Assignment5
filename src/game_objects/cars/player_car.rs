use std::{f32, rc::Rc};

use glow::Context;
use nalgebra::Vector3;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    core::{
        constants::{
            FINISH_LINE_X_START, FINISH_LINE_X_STOP, FINISH_LINE_Z, HALF_RING_LINE_Z,
            HALF_RING_X_START, HALF_RING_X_STOP,
        },
        game::Game,
        game_object::GameObject, color::Color,
    },
    game_objects::cars::car::ViewState,
    network::server_connection::NetworkEvent,
    objects::mesh_model::MeshModel,
};

use super::{super::track::track_segment::TRACK_ELEVATION, car::Car};

const LOOK_DIST: f32 = 0.9;

enum BrakingState {
    None,
    Braking,
    Reversing,
}

pub struct PlayerCar<'a> {
    car: Car<'a>,
    lap_half_ring_complete: bool,
    braking_state: BrakingState,
    joystic_braking_state: BrakingState,
}

impl<'a> PlayerCar<'a> {
    pub fn new(
        car_model: Rc<MeshModel<'a>>,
        wheel_model: Rc<MeshModel<'a>>,
        gl: &'a Context,
        game: &Game,
    ) -> PlayerCar<'a> {
        let car = Car::new(true, car_model, wheel_model, gl, game);

        let mut lights = game.lights.borrow_mut();
        lights.add_light("PLAYER_CAR");
        lights.set_light_diffuse("PLAYER_CAR", &Color::new(0.89, 0.91, 1.00));
        lights
            .set_light_ambient("PLAYER_CAR", &Color::with_alpha(0.0, 0.0, 0.0, 0.0));
        lights.set_light_specular("PLAYER_CAR", &Color::new(0.89, 0.91, 1.00));
        lights.set_light_max_radius("PLAYER_CAR", 50.0);

        PlayerCar {
            car,
            lap_half_ring_complete: false,
            braking_state: BrakingState::None,
            joystic_braking_state: BrakingState::None,
        }
    }

    fn i16_range_to_float(&self, value: i16) -> f32 {
        (value as f32 - i16::MIN as f32) / (i16::MAX as f32 + -(i16::MIN as f32))
    }

    fn handle_joystick_controls(&mut self, game: &Game) {
        let joystick = match &game.joystick {
            Some(j) if j.attached() => j,
            _ => return,
        };

        let rt_value = joystick.axis(5).unwrap_or(i16::MIN);
        self.car
            .set_throttle(self.i16_range_to_float(rt_value) * 100.0);

        // Braking
        let lt_value = joystick.axis(4).unwrap_or(i16::MIN);
        let brake_value = self.i16_range_to_float(lt_value) * 100.0;
        if brake_value > 5.0 {
            use BrakingState::*;
            match self.joystic_braking_state {
                Braking => {
                    if self.car.car_state().velocity_wc.norm() < 0.3 {
                        self.joystic_braking_state = Reversing;
                    }

                    self.car.set_brake(brake_value);
                }
                Reversing => {
                    self.car.set_brake(0.0);
                    self.car.set_throttle(brake_value * 0.2);
                    self.car.set_reverse(true);
                }
                None => {
                    self.joystic_braking_state = Braking;
                }
            }
        } else if !matches!(self.joystic_braking_state, BrakingState::None) {
            if matches!(self.joystic_braking_state, BrakingState::Reversing) {
                self.car.car_state_mut().velocity_wc = -self.car.car_state().velocity_wc;
            }

            self.car.set_brake(0.0);
            self.car.set_reverse(false);
            self.joystic_braking_state = BrakingState::None;
        }

        let left_x_axis = joystick.axis(0).unwrap_or(0);
        // let left_y_axis = joystick.axis(1).unwrap_or(0);
        self.car.set_steering_angle(
            -(self.i16_range_to_float(left_x_axis) * 2.0 * (0.25 * f32::consts::PI / 4.0)
                - (0.25 * f32::consts::PI / 4.0)),
        );
    }

    fn spawn_position(player_id: u8) -> Vector3<f32> {
        match player_id {
            1 => Vector3::new(-4.5, TRACK_ELEVATION, 120.0),
            2 => Vector3::new(5.0, TRACK_ELEVATION, 100.0),
            3 => Vector3::new(-4.5, TRACK_ELEVATION, 80.0),
            4 => Vector3::new(5.0, TRACK_ELEVATION, 60.0),
            5 => Vector3::new(-4.5, TRACK_ELEVATION, 40.0),
            6 => Vector3::new(5.0, TRACK_ELEVATION, 20.0),
            _ => Vector3::new(-4.5, TRACK_ELEVATION, 0.0),
        }
    }
}

impl<'a> GameObject<'a> for PlayerCar<'a> {
    fn on_event(&mut self, _game: &Game, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                use Keycode::*;
                match key {
                    W => {
                        self.car.set_throttle(100.0);
                    }
                    S => {
                        use BrakingState::*;
                        match self.braking_state {
                            Braking => {
                                if self.car.car_state().velocity_wc.norm() < 0.3 {
                                    self.braking_state = Reversing;
                                }

                                self.car.set_throttle(0.0);
                                self.car.set_brake(100.0);
                            }
                            Reversing => {
                                self.car.set_brake(0.0);
                                self.car.set_throttle(20.0);
                                self.car.set_reverse(true);
                            }
                            None => {
                                self.braking_state = Braking;
                            }
                        }
                    }
                    A => {
                        self.car.set_steering_angle((f32::consts::PI / 4.0) * 0.25);
                    }
                    D => self
                        .car
                        .set_steering_angle((-std::f32::consts::PI / 4.0) * 0.25),
                    Space => {
                        self.car.set_handbrake(true);
                    }
                    V => {
                        let view_state = match self.car.view_state() {
                            ViewState::ThirdPerson => ViewState::FirstPerson,
                            ViewState::FirstPerson => ViewState::ThirdPerson,
                        };
                        self.car.set_view_state(view_state);
                    }
                    _ => (),
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                use Keycode::*;
                match key {
                    W => {
                        self.car.set_throttle(0.0);
                    }
                    S => {
                        if matches!(self.braking_state, BrakingState::Reversing) {
                            self.car.car_state_mut().velocity_wc = -self.car.car_state().velocity_wc;
                        }

                        self.braking_state = BrakingState::None;
                        self.car.set_brake(0.0);
                        self.car.set_throttle(0.0);
                        self.car.set_reverse(false);
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
                    P => {
                        let pos = self.car.position();
                        println!("Pos: {}, {}, {}", pos.x, pos.y, pos.z);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        let current_pos = self.car.position();
        let future_pos = self
            .car
            .car_state()
            .peek_time_step(game.delta_time, self.car.handbrake(), self.car.handbrake(), self.car.reverse())
            .position_wc;

        if self.lap_half_ring_complete
            && current_pos.x > FINISH_LINE_X_START
            && current_pos.x < FINISH_LINE_X_STOP
            && FINISH_LINE_Z >= current_pos.z
            && FINISH_LINE_Z <= future_pos.z
        {
            log::debug!("Lap!");
            if game.server_connection.is_multiplayer() {
                game.server_connection.send_lap_complete();
            }
            self.lap_half_ring_complete = false;
        }

        if current_pos.x > HALF_RING_X_START
            && current_pos.x < HALF_RING_X_STOP
            && HALF_RING_LINE_Z <= current_pos.z
            && HALF_RING_LINE_Z >= future_pos.z
        {
            self.lap_half_ring_complete = true;
        }

        self.handle_joystick_controls(game);

        self.car.update(game, gl);

        // Send status update
        if game.server_connection.is_multiplayer() {
            loop {
                let mut game_events = game.server_connection.game_events.borrow_mut();
                let event = game_events.front();

                match event {
                    Some(NetworkEvent::MoveToStartPos) => {
                        self.car.reset_physics();
                        self.car.set_position(PlayerCar::spawn_position(
                            game.server_connection.player_id().unwrap_or(1),
                        ));
                        game_events.pop_front();
                    }
                    _ => break,
                }
            }

            game.server_connection.send_status_update(
                self.car.position(),
                self.car.angle(),
                self.car.steering_angle(),
            );
        }

        // Update lights
        let pos = self.car.light_position();
        game.lights.borrow_mut().set_light_position("PLAYER_CAR", &Vector3::new(pos.x, pos.y, pos.z));

        // Update camera pos
        let mut view_matrix = game.view_matrix.borrow_mut();
        let ang_sin = self.car.angle().sin();
        let ang_cos = self.car.angle().cos();

        use ViewState::*;
        let eye = match self.car.view_state() {
            FirstPerson => self.car.position() + Vector3::new(ang_sin * -1.8, 1.2, ang_cos * -1.8),
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
