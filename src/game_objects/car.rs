use glow::*;
use nalgebra::Vector3;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{core::{obj_loader::load_obj_file, game_object::{GameObject, CollisionInfo}, game::Game}, objects::mesh_model::MeshModel, utils::car_sim};

const LOOK_DIST: f32 = 0.9;

enum MovingDirection {
    Stop,
    Forward,
    Backward
}

pub struct Car<'a> {
    car_model: MeshModel<'a>,
    wheel_model: MeshModel<'a>,
    car_pos: Vector3<f32>,
    car_velocity: Vector3<f32>,
    move_direction: MovingDirection,
    rotation: f32,
    turning_angle: f32,
    y_velocity: f32
}

impl<'a> Car<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Car<'a> {
        let car_model = load_obj_file("./models", "car.obj", gl, game).expect("Failed to load car model");
        let wheel_model = load_obj_file("./models", "wheel.obj", gl, game).expect("Failed to load wheel model");
        // let tex_id = game.load_texture("./models/textures/Ferrari_246_F1_Low_Car_BaseColor.png");

        Car { car_model, wheel_model, move_direction: MovingDirection::Stop, car_pos: Vector3::zeros(), car_velocity: Vector3::zeros(), rotation: 0.0, turning_angle: 0.0, y_velocity: 0.0 }
    }

    fn check_collision(&mut self, game: &Game) {
        for object in &game.game_objects {
            if self as *const _ as *const usize == object.as_ptr() as *const _ as *const usize  {
                continue;
            }

            use CollisionInfo::*;
            match object.borrow().collision_info() {
                YCollision(y) => {
                    if self.car_pos.y <= y {
                        self.car_pos.y = y;
                        self.y_velocity = 0.0;
                    }
                },
                NoCollision => ()
            }
        }
    }

    fn update_gravity(&mut self, game: &Game) {
        self.car_pos.y += self.y_velocity;
        self.y_velocity -= 9.8 * 1.7 * game.delta_time;
    }
}

impl<'a> GameObject<'a> for Car<'a> {
    fn on_event(&mut self, game: &Game, event: &Event) {
        match event {
            Event::KeyDown { keycode:  Some(key @ (Keycode::W | Keycode::S)), .. } => {
                use Keycode::*;
                self.move_direction = match key {
                    W => MovingDirection::Forward,
                    S => MovingDirection::Backward,
                    _ => MovingDirection::Stop
                };
            },
            Event::KeyUp { keycode:  Some(Keycode::W | Keycode::S), .. } => {
                self.move_direction = MovingDirection::Stop;
                self.car_velocity = Vector3::zeros();
            },
            Event::KeyDown { keycode:  Some(key @ (Keycode::Left | Keycode::Right)), .. } => {
                use Keycode::*;
                self.turning_angle = match key {
                    Left => 1.0,
                    Right => -1.0,
                    _ => 0.0
                };
            },
            Event::KeyUp { keycode:  Some(Keycode::Left | Keycode::Right), .. } => {
                self.turning_angle = 0.0;
            },
            _ => ()
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        return;
        let mut view_matrix = game.view_matrix.borrow_mut();
        let ang_sin = self.rotation.sin();
        let ang_cos = self.rotation.cos();

        // Move car

        if self.turning_angle != 0.0 {
            self.rotation += self.turning_angle * game.delta_time;
        }

        use MovingDirection::*;
        match self.move_direction {
            Forward => {
                (self.car_pos, self.car_velocity) = car_sim::accelerate_time_step(&self.car_pos, &self.car_velocity, &Vector3::new(ang_sin, 0.0, ang_cos), game.delta_time);

            },
            Backward => self.car_pos += Vector3::new(ang_sin, 0.0, ang_cos) * game.delta_time * -25.0,
            Stop => ()
        }

        self.check_collision(game);
        self.update_gravity(game);

        // Update camera pos
        let eye = self.car_pos + Vector3::new(ang_sin * -20.0, 8.0, ang_cos * -20.0);
        let center = eye + Vector3::new(ang_sin * LOOK_DIST, 0.0, ang_cos * LOOK_DIST);
        view_matrix.look(eye, center, Vector3::new(0.0, 1.0, 0.0));
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        model_matrix.push_stack();
        model_matrix.add_translate(self.car_pos.x, self.car_pos.y, self.car_pos.z);
        model_matrix.add_scale(5.0, 5.0, 5.0);
        model_matrix.add_rotation(0.0, self.rotation, 0.0);

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.car_model.draw(&game.shader);

        model_matrix.push_stack();
        model_matrix.add_translate(0.4, 0.0, 0.9);
        model_matrix.add_rotation(0.0, 90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(-0.4, 0.0, 0.8);
        model_matrix.add_rotation(0.0, -90.0f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.wheel_model.draw(&game.shader);
        model_matrix.pop_stack();

        model_matrix.push_stack();
        model_matrix.add_translate(0.4, 0.0, -0.5);
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