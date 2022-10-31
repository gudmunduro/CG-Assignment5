use std::{f32, rc::Rc};

use glow::*;
use nalgebra::{Vector3, Vector4, Vector2};
use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    core::{
        game::Game,
        game_object::{Collider, GameObject},
        obj_loader::load_obj_file, color::Color,
    },
    objects::mesh_model::MeshModel,
    utils::{car_sim::{self, CarState}, line_contains_point},
};

pub enum ViewState {
    ThirdPerson,
    FirstPerson,
}

pub struct Car<'a> {
    car_model: Rc<MeshModel<'a>>,
    wheel_model: Rc<MeshModel<'a>>,
    car_state: CarState,
    throttle_state: bool,
    y_velocity: f32,
    handbrake: bool,
    wheel_rotation: f32,
    view_state: ViewState,
    enable_collision_check: bool
}

impl<'a> Car<'a> {
    pub fn new(enable_collision_check: bool, car_model: Rc<MeshModel<'a>>, wheel_model: Rc<MeshModel<'a>>, gl: &'a Context, game: &Game) -> Car<'a> {
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
            enable_collision_check
        }
    }

    pub fn car_cube(&self, game: &Game) -> (f32, f32, f32, f32, f32, f32) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        model_matrix.add_translate(
            self.car_state.position_wc.x,
            self.car_state.position_wc.y,
            self.car_state.position_wc.z,
        );
        model_matrix.add_rotation(0.0, self.car_state.angle, 0.0);
        model_matrix.add_scale(5.0, 3.0, 10.0);

        let local_max = Vector4::new(0.5, 0.5, 0.5, 1.0);
        let local_min = Vector4::new(-0.5, -0.5, -0.5, 1.0);

        let max = model_matrix.matrix * local_max;
        let min = model_matrix.matrix * local_min;

        model_matrix.pop_stack();

        return (
            min.x.min(max.x),
            min.y.min(max.y),
            min.z.min(max.z),
            max.x.max(min.x),
            max.y.max(min.y),
            max.z.max(min.z),
        );
    }

    fn full_car_cube(&self, game: &Game, custom_car_state: Option<&CarState>) -> [Vector3<f32>; 8] {
        let car_state = custom_car_state.unwrap_or(&self.car_state);

        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        model_matrix.add_translate(
            car_state.position_wc.x,
            car_state.position_wc.y,
            car_state.position_wc.z,
        );
        model_matrix.add_rotation(0.0, car_state.angle, 0.0);
        model_matrix.add_scale(5.0, 3.0, 10.0);

        let bottom_left_inner_local = Vector4::new(-0.5, -0.5, -0.5, 1.0);
        let bottom_right_inner_local = Vector4::new(0.5, -0.5, -0.5, 1.0);
        let top_left_inner_local = Vector4::new(-0.5, 0.5, -0.5, 1.0);
        let top_right_inner_local = Vector4::new(0.5, 0.5, -0.5, 1.0);
        let bottom_left_outer_local = Vector4::new(-0.5, -0.5, 0.5, 1.0);
        let bottom_right_outer_local = Vector4::new(0.5, -0.5, 0.5, 1.0);
        let top_left_outer_local = Vector4::new(-0.5, 0.5, 0.5, 1.0);
        let top_right_outer_local = Vector4::new(0.5, 0.5, 0.5, 1.0);

        let bottom_left_inner = (model_matrix.matrix * bottom_left_inner_local).xyz();
        let bottom_right_inner = (model_matrix.matrix * bottom_right_inner_local).xyz();
        let bottom_left_outer = (model_matrix.matrix * bottom_left_outer_local).xyz();
        let bottom_right_outer = (model_matrix.matrix * bottom_right_outer_local).xyz();

        let top_left_inner = (model_matrix.matrix * top_left_inner_local).xyz();
        let top_right_inner = (model_matrix.matrix * top_right_inner_local).xyz();
        let top_left_outer = (model_matrix.matrix * top_left_outer_local).xyz();
        let top_right_outer = (model_matrix.matrix * top_right_outer_local).xyz();

        model_matrix.pop_stack();

        [
            bottom_left_inner,
            bottom_right_inner,
            bottom_left_outer,
            bottom_right_outer,
            top_left_inner,
            top_right_inner,
            top_left_outer,
            top_right_outer,
        ]
    }

    fn check_collision(&mut self, info: &Collider, game: &Game) {
        use Collider::*;
        match info {
            &HeightCollider(y) => {
                let (_, c_min_y, ..) = self.car_cube(game);
                if c_min_y <= y {
                    self.car_state.position_wc.y = y + 1.5;
                    self.y_velocity = 0.0;
                }
            }
            &BoxCollider(min_x, min_y, min_z, max_x, max_y, max_z) => {
                let (c_min_x, c_min_y, c_min_z, c_max_x, c_max_y, c_max_z) = self.car_cube(game);

                let perform_collision_detect = c_min_x <= max_x
                    && c_max_x >= min_x
                    && c_min_y <= max_y
                    && c_max_y >= min_y
                    && c_min_z <= max_z
                    && c_max_z >= min_z;

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
            InfiniteYPlaneCollider(p0, p1) => {
                let corners = self.full_car_cube(game, None);
                let mut car_state_predicted = self.car_state.clone();
                car_state_predicted.perform_physics_time_step(game.delta_time, self.handbrake, self.handbrake);
                let future_corners = self.full_car_cube(game, Some(&car_state_predicted));

                for (corner, f_corner) in corners[..4].iter().zip(&future_corners[..4]) {
                    let v = (p1 - p0).xz();

                    let a_mat = corner.xz();
                    let b_mat = p0.xz();
                    let c = (f_corner.xz() - corner.xz()) / game.delta_time;
                    let n = Vector2::new(-v.y, v.x);

                    let t_hit = (n.dot(&(b_mat-a_mat))) / (n.dot(&c));
                    let p_hit = a_mat + t_hit * c;

                    if line_contains_point(&p0.xz(), &p1.xz(), &p_hit) && t_hit <= game.delta_time && t_hit > 0.0 {
                        let reflected = c - ((2.0 * (c.dot(&n))) / (n.dot(&n))) * n;

                        self.car_state.velocity_wc.x = reflected.x;
                        self.car_state.velocity_wc.z = reflected.y;
                    }
                }
            }
            MultiCollider(c) => c.iter().for_each(|info| self.check_collision(&info, game)),
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

    pub fn set_handbrake(&mut self, value: bool) {
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
    fn on_event(&mut self, game: &Game, event: &Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {
        if self.enable_collision_check {
            self.check_all_collision(game);
        }

        self.car_state
            .perform_physics_time_step(game.delta_time, self.handbrake, self.handbrake);

        self.wheel_rotation += self.car_state.wheel_rotation_speed;
        if self.wheel_rotation >= 2.0 * f32::consts::PI {
            self.wheel_rotation = self.wheel_rotation % (2.0 * f32::consts::PI);
        }

        self.update_gravity(game);
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
