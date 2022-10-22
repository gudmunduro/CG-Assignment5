use glow::*;
use nalgebra::Vector3;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{core::{obj_loader::load_obj_file, game_object::{GameObject, CollisionInfo}, game::Game}, objects::mesh_model::MeshModel, utils::car_sim::{self, CarState}};

const LOOK_DIST: f32 = 0.9;

pub struct Car<'a> {
    car_model: MeshModel<'a>,
    wheel_model: MeshModel<'a>,
    car_state: CarState,
    throttle_state: bool,
    y_velocity: f32,
    handbrake: bool
}

impl<'a> Car<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Car<'a> {
        let car_model = load_obj_file("./models", "car.obj", gl, game).expect("Failed to load car model");
        let wheel_model = load_obj_file("./models", "wheel.obj", gl, game).expect("Failed to load wheel model");
        // let tex_id = game.load_texture("./models/textures/Ferrari_246_F1_Low_Car_BaseColor.png");

        Car { car_model, wheel_model, throttle_state: false, car_state: CarState::new(), y_velocity: 0.0, handbrake: false }
    }

    fn check_collision(&mut self, game: &Game) {
        for object in &game.game_objects {
            if self as *const _ as *const usize == object.as_ptr() as *const _ as *const usize  {
                continue;
            }

            use CollisionInfo::*;
            match object.borrow().collision_info() {
                YCollision(y) => {
                    /*if self.car_pos.y <= y {
                        self.car_pos.y = y;
                        self.y_velocity = 0.0;
                    }*/
                },
                NoCollision => ()
            }
        }
    }

    fn update_gravity(&mut self, game: &Game) {
        // self.car_pos.y += self.y_velocity;
        self.y_velocity -= 9.8 * 1.7 * game.delta_time;
    }
}

impl<'a> GameObject<'a> for Car<'a> {
    fn on_event(&mut self, game: &Game, event: &Event) {
        match event {
            Event::KeyDown { keycode:  Some(key @ (Keycode::W | Keycode::S)), .. } => {
                if *key == Keycode::W {
                    self.car_state.throttle = 100.0;
                }
                else if *key == Keycode::S {
                    self.car_state.throttle = 0.0;
                    self.car_state.brake = 100.0;
                }
            },
            Event::KeyUp { keycode:  Some(key @ (Keycode::W | Keycode::S)), .. } => {
                if *key == Keycode::W {
                    self.car_state.throttle = 0.0;
                }
                else if *key == Keycode::S {
                    self.car_state.brake = 0.0;
                }
            },
            Event::KeyDown { keycode:  Some(key @ (Keycode::A | Keycode::D)), .. } => {
                use Keycode::*;
                self.car_state.steering_angle = match key {
                    A => {
                        (std::f32::consts::PI / 4.0) * 0.25
                    }
                    D => {
                        (-std::f32::consts::PI / 4.0) * 0.25
                    }
                    _ => 0.0,
                };
            },
            Event::KeyUp { keycode:  Some(Keycode::A | Keycode::D), .. } => {
                self.car_state.steering_angle = 0.0;
            },
            _ => ()
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        let mut view_matrix = game.view_matrix.borrow_mut();
        let ang_sin = self.car_state.angle.sin();
        let ang_cos = self.car_state.angle.cos();

        self.car_state.perform_physics_time_step(game.delta_time, false, false);

        // Update camera pos
        let eye = self.car_state.position_wc + Vector3::new(ang_sin * -20.0, 8.0, ang_cos * -20.0);
        let center = eye + Vector3::new(ang_sin * LOOK_DIST, 0.0, ang_cos * LOOK_DIST);
        view_matrix.look(eye, center, Vector3::new(0.0, 1.0, 0.0));
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        // Car
        model_matrix.push_stack();
        model_matrix.add_translate(self.car_state.position_wc.x, self.car_state.position_wc.y, self.car_state.position_wc.z);
        model_matrix.add_scale(5.0, 5.0, 5.0);
        model_matrix.add_rotation(0.0, self.car_state.angle, 0.0);

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.car_model.draw(&game.shader);

        // Front wheels
        model_matrix.push_stack();
        model_matrix.add_translate(0.4, 0.0, 0.8);
        model_matrix.add_rotation(0.0, 90.0f32.to_radians() + self.car_state.steering_angle, 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(-0.4, 0.0, 0.8);
        model_matrix.add_rotation(0.0, -90.0f32.to_radians() + self.car_state.steering_angle, 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        // Rear wheels
        model_matrix.push_stack();
        model_matrix.add_translate(0.4, 0.0, -0.49);
        model_matrix.add_rotation(0.0, 90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(-0.4, 0.0, -0.6);
        model_matrix.add_rotation(0.0, -90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.pop_stack();


    }
}