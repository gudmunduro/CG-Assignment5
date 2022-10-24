use std::f32;

use glow::*;
use nalgebra::{Vector3, Vector4};
use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    core::{
        game::Game,
        game_object::{CollisionInfo, GameObject},
        obj_loader::load_obj_file,
    },
    objects::mesh_model::MeshModel,
    utils::car_sim::{self, CarState},
};

const LOOK_DIST: f32 = 0.9;

enum ViewState {
    ThirdPerson,
    FirstPerson,
}

pub struct Car<'a> {
    car_model: MeshModel<'a>,
    wheel_model: MeshModel<'a>,
    car_state: CarState,
    throttle_state: bool,
    y_velocity: f32,
    handbrake: bool,
    wheel_rotation: f32,
    view_state: ViewState,
}

impl<'a> Car<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Car<'a> {
        let car_model =
            load_obj_file("./models", "car.obj", gl, game).expect("Failed to load car model");
        let wheel_model =
            load_obj_file("./models", "wheel2.obj", gl, game).expect("Failed to load wheel model");
        let mut car_state = CarState::new();
        car_state.position_wc.y = 40.0;

        Car {
            car_model,
            wheel_model,
            throttle_state: false,
            car_state,
            y_velocity: 0.0,
            handbrake: false,
            wheel_rotation: 0.0,
            view_state: ViewState::ThirdPerson,
        }
    }

    fn car_cube(&self, game: &Game) -> (f32, f32, f32, f32, f32, f32) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        model_matrix.add_translate(self.car_state.position_wc.x, self.car_state.position_wc.y, self.car_state.position_wc.z);
        model_matrix.add_rotation(0.0, self.car_state.angle, 0.0);
        model_matrix.add_scale(5.0, 3.0, 10.0);
        
        let local_max = Vector4::new(0.5, 0.5, 0.5, 1.0);
        let local_min = Vector4::new(-0.5, -0.5, -0.5, 1.0);

        let max =  model_matrix.matrix * local_max;
        let min = model_matrix.matrix * local_min;

        model_matrix.pop_stack();

        return (min.x.min(max.x), min.y.min(max.y), min.z.min(max.z), max.x.max(min.x), max.y.max(min.y), max.z.max(min.z));
    }

    fn check_collision(&mut self, info: &CollisionInfo, game: &Game) {
        use CollisionInfo::*;
        match info {
            &YCollision(y) => {
                let (_, c_min_y, ..) = self.car_cube(game);
                if c_min_y <= y {
                    self.car_state.position_wc.y = y + 1.5;
                    self.y_velocity = 0.0;
                }
            }
            &BoxCollision(min_x, min_y, min_z, max_x, max_y, max_z) => {
                let (c_min_x, c_min_y, c_min_z, c_max_x, c_max_y, c_max_z) = self.car_cube(game);

                let perform_collision_detect = c_min_x <= max_x &&
                    c_max_x >= min_x &&
                    c_min_y <= max_y &&
                    c_max_y >= min_y &&
                    c_min_z <= max_z &&
                    c_max_z >= min_z;

                if perform_collision_detect {
                    let mut closest_x = self.car_state.position_wc.x;
                    if self.car_state.position_wc.x > max_x {
                        closest_x = max_x + 2.5;
                    } else if self.car_state.position_wc.x < min_x {
                        closest_x = min_x - 2.5;
                    }

                    let mut closest_y = self.car_state.position_wc.y;
                    if self.car_state.position_wc.y > max_y {
                        self.y_velocity = 0.0;
                        closest_y = max_y + 1.5;
                        // println!("Collided with y");
                    } else if self.car_state.position_wc.y < min_y {
                        self.y_velocity = -1.0;
                        closest_y = min_y - 1.5;
                        // println!("Collided with -y");
                    }

                    let mut closest_z = self.car_state.position_wc.z;
                    if self.car_state.position_wc.z > max_z {
                        closest_z = max_z + 5.0;
                    } else if self.car_state.position_wc.z < min_z {
                        closest_z = min_z - 5.0;
                    }

                    self.car_state.position_wc.x = closest_x;
                    self.car_state.position_wc.y = closest_y;
                    self.car_state.position_wc.z = closest_z;
                }
            }
            MultiCollision(c) => c.iter().for_each(|info| self.check_collision(&info, game)),
            NoCollision => (),
        }
    }

    fn check_all_collision(&mut self, game: &Game) {
        for object in &game.game_objects {
            if self as *const _ as *const usize == object.as_ptr() as *const _ as *const usize {
                continue;
            }

            self.check_collision(&object.borrow().collision_info(), game);
        }
    }

    fn update_gravity(&mut self, game: &Game) {
        self.car_state.position_wc.y += self.y_velocity * game.delta_time;
        self.y_velocity -= 9.8 * 1.7 * game.delta_time;
    }
}

impl<'a> GameObject<'a> for Car<'a> {
    fn on_event(&mut self, game: &Game, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key @ (Keycode::W | Keycode::S)),
                ..
            } => {
                if *key == Keycode::W {
                    self.car_state.throttle = 100.0;
                } else if *key == Keycode::S {
                    self.car_state.throttle = 0.0;
                    self.car_state.brake = 100.0;
                }
            }
            Event::KeyUp {
                keycode: Some(key @ (Keycode::W | Keycode::S)),
                ..
            } => {
                if *key == Keycode::W {
                    self.car_state.throttle = 0.0;
                } else if *key == Keycode::S {
                    self.car_state.brake = 0.0;
                }
            }
            Event::KeyDown {
                keycode: Some(key @ (Keycode::A | Keycode::D)),
                ..
            } => {
                use Keycode::*;
                self.car_state.steering_angle = match key {
                    A => (std::f32::consts::PI / 4.0) * 0.25,
                    D => (-std::f32::consts::PI / 4.0) * 0.25,
                    _ => 0.0,
                };
            }
            Event::KeyUp {
                keycode: Some(Keycode::A | Keycode::D),
                ..
            } => {
                self.car_state.steering_angle = 0.0;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.handbrake = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.handbrake = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => {
                self.view_state = if matches!(self.view_state, ViewState::ThirdPerson) {
                    ViewState::FirstPerson
                } else {
                    ViewState::ThirdPerson
                };
            }
            Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                self.y_velocity = 20.0;
            }
            _ => (),
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        let mut view_matrix = game.view_matrix.borrow_mut();
        let ang_sin = self.car_state.angle.sin();
        let ang_cos = self.car_state.angle.cos();

        self.car_state
            .perform_physics_time_step(game.delta_time, self.handbrake, self.handbrake);

        self.wheel_rotation += self.car_state.wheel_rotation_speed;
        if self.wheel_rotation >= 2.0 * f32::consts::PI {
            self.wheel_rotation = self.wheel_rotation % (2.0 * f32::consts::PI);
        }

        self.update_gravity(game);
        self.check_all_collision(game);

        // Update camera pos
        use ViewState::*;
        let eye = match self.view_state {
            FirstPerson => {
                self.car_state.position_wc + Vector3::new(ang_sin * -1.8, 1.2, ang_cos * -1.8)
            }
            ThirdPerson => {
                self.car_state.position_wc + Vector3::new(ang_sin * -20.0, 6.0, ang_cos * -20.0)
            }
        };
        let center = eye + Vector3::new(ang_sin * LOOK_DIST, 0.0, ang_cos * LOOK_DIST);
        view_matrix.look(eye, center, Vector3::new(0.0, 1.0, 0.0));
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        // Car
        model_matrix.push_stack();
        model_matrix.add_translate(
            self.car_state.position_wc.x,
            self.car_state.position_wc.y,
            self.car_state.position_wc.z,
        );
        model_matrix.add_scale(5.0, 5.0, 5.0);
        model_matrix.add_rotation(0.0, self.car_state.angle, 0.0);

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.car_model.draw(&game.shader);

        // Front wheels
        model_matrix.push_stack();
        model_matrix.add_translate(0.4, -0.1, 0.8);
        model_matrix.add_rotation(
            0.0,
            90.0f32.to_radians() + self.car_state.steering_angle,
            0.0,
        );
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(-0.4, -0.1, 0.8);
        model_matrix.add_rotation(
            0.0,
            -90.0f32.to_radians() + self.car_state.steering_angle,
            0.0,
        );
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        // Rear wheels
        model_matrix.push_stack();
        model_matrix.add_translate(0.4, -0.1, -0.6);
        model_matrix.add_rotation(self.wheel_rotation, 90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(-0.4, -0.1, -0.6);
        model_matrix.add_rotation(self.wheel_rotation, -90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.pop_stack();
    }
}
