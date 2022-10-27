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

pub enum ViewState {
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

    pub fn throttle(&self) -> f32 {
        self.car_state.throttle
    }

    pub fn set_throttle(&mut self, value: f32) {
        self.car_state.throttle = value;
    }

    pub fn brake(&self) -> f32 {
        self.car_state.brake
    }

    pub fn set_brake(&mut self, value: f32) {
        self.car_state.brake = value;
    }

    pub fn steering_angle(&self) -> f32 {
        self.car_state.steering_angle
    }

    pub fn set_steering_angle(&mut self, value: f32) {
        self.car_state.steering_angle = value;
    }

    pub fn y_velocity(&self) -> f32 {
        self.y_velocity
    }

    pub fn set_y_velocity(&mut self, value: f32) {
        self.y_velocity = value;
    }

    pub fn view_state(&self) -> &ViewState {
        &self.view_state
    }

    pub fn set_view_state(&mut self, view_state: ViewState) {
        self.view_state = view_state;
    }

    pub fn handbrake(&self) -> bool {
        self.handbrake
    }

    pub fn set_handbrake(&mut self, value: bool)
    {
        self.handbrake = value;
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.car_state.position_wc
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.car_state.position_wc = position;
    }

    pub fn angle(&self) -> f32 {
        self.car_state.angle
    }

    pub fn set_angle(&mut self, value: f32) {
        self.car_state.angle = value;
    }
}

impl<'a> GameObject<'a> for Car<'a> {
    fn on_event(&mut self, game: &Game, event: &Event) {
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        self.car_state
            .perform_physics_time_step(game.delta_time, self.handbrake, self.handbrake);

        self.wheel_rotation += self.car_state.wheel_rotation_speed;
        if self.wheel_rotation >= 2.0 * f32::consts::PI {
            self.wheel_rotation = self.wheel_rotation % (2.0 * f32::consts::PI);
        }

        self.update_gravity(game);
        self.check_all_collision(game);

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
