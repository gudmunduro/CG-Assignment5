use crate::{core::{
    matrices::{ModelMatrix, ProjectionMatrix, ViewMatrix},
    shader::Shader3D,
}, objects::cube::Cube};
use glow::*;
use nalgebra::Vector3;
use sdl2::{
    video::{GLContext, Window},
    EventPump, event::Event, keyboard::Keycode,
};

use super::{constants::{W_WIDTH, W_HEIGHT}, color::Color};

enum ArrowDir {
    Up,
    Down,
    Left,
    Right,
    None
}

pub struct Game<'a> {
    gl: &'a Context,
    window: &'a Window,
    events_loop: &'a mut EventPump,
    gl_context: &'a GLContext,
    shader: &'a Shader3D<'a>,
    model_matrix: &'a mut ModelMatrix,
    view_matrix: &'a mut ViewMatrix,
    projection_matrix: &'a mut ProjectionMatrix,
    cube: &'a Cube<'a>,
    moving_foward: bool,
    moving_backward: bool,
    arrow_direction: ArrowDir
}

impl<'a> Game<'a> {
    pub fn new(
        gl: &'a Context,
        window: &'a Window,
        events_loop: &'a mut EventPump,
        gl_context: &'a GLContext,
        shader: &'a Shader3D<'a>,
        model_matrix: &'a mut ModelMatrix,
        view_matrix: &'a mut ViewMatrix,
        projection_matrix: &'a mut ProjectionMatrix,
        cube: &'a Cube,
    ) -> Game<'a> {
        shader.use_program();
        shader.set_view_matrix(view_matrix.get_matrix().as_slice());
        projection_matrix.set_perspective(80.0, W_WIDTH as f32 / W_HEIGHT as f32, 0.5, 80.0);
        shader.set_projection_matrix(projection_matrix.get_matrix().as_slice());

        Game {
            gl,
            window,
            events_loop,
            gl_context,
            shader,
            model_matrix,
            view_matrix,
            projection_matrix,
            cube,
            moving_foward: false,
            moving_backward: false,
            arrow_direction: ArrowDir::None
        }
    }

    pub fn update(&mut self) {
        if self.moving_foward {
            self.view_matrix.slide(0.0, 0.0, -1.0, Vector3::zeros(), Vector3::zeros(), self.view_matrix.n);
        }

        if self.moving_backward {
            self.view_matrix.slide(0.0, 0.0, 1.0, Vector3::zeros(), Vector3::zeros(), self.view_matrix.n);
        }

        let rot_speed = 1.0;
        use ArrowDir::*;
        match self.arrow_direction {
            Up => self.view_matrix.pitch(rot_speed),
            Down => self.view_matrix.pitch(-rot_speed),
            Left => self.view_matrix.yaw(rot_speed),
            Right => self.view_matrix.yaw(-rot_speed),
            None => ()
        }
    }

    pub fn display(&mut self) {
        unsafe {
            self.gl.enable(DEPTH_TEST);
            self.gl.clear_color(0.03, 0.04, 0.13, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.viewport(0, 0, W_WIDTH as i32, W_HEIGHT as i32);
        }

        self.shader.set_view_matrix(self.view_matrix.get_matrix().as_slice());
        self.shader.set_projection_matrix(self.projection_matrix.get_matrix().as_slice());

        self.model_matrix.load_identity();

        self.shader.set_light_position(&[0.0, 0.0, 0.0, 1.0]);
        self.shader.set_light_diffuse(&[0.5, 0.5, 0.5, 0.0]);
        self.shader.set_light_ambient(&[0.5, 0.5, 0.5, 0.0]);
        self.shader.set_light_specular(&[1.0, 1.0, 1.0, 0.0]);
        self.shader.set_eye_position(self.view_matrix.eye.x, self.view_matrix.eye.y, self.view_matrix.eye.z);

        self.shader.set_material_ambient(0.5);
        self.shader.set_material_diffuse(&Color::new(0.0, 0.18, 0.02));
        self.shader.set_material_specular(&Color::new(1.0, 1.0, 1.0));
        self.shader.set_shininess(3.0);


        self.model_matrix.push_stack();
        self.model_matrix.add_translate(0.0, 0.0, -8.0);
        self.model_matrix.add_scale(5.0, 5.0, 5.0);

        self.shader.set_model_matrix(self.model_matrix.matrix.as_slice());
        self.cube.draw(self.shader);
        self.model_matrix.pop_stack();

        self.window.gl_swap_window();
    }

    pub fn main(&mut self) {
        unsafe {
            let mut running = true;
            while running {
                {
                    for event in self.events_loop.poll_iter() {
                        match event {
                            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
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
                            _ => {}
                        }
                    }
                }

                self.update();
                self.display();
            }
        }
    }
}
