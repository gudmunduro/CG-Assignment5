use std::{time::Instant, cell::RefCell, path::Path, slice, collections::VecDeque};

use crate::{core::{
    matrices::{ModelMatrix, ProjectionMatrix, ViewMatrix},
    shader::Shader3D,
}, objects::cube::Cube, game_objects::{car::Car, freecam_controller::FreecamController, ground::Ground, track_segment::TrackSegment, track::Track, player_car::PlayerCar, network_car::NetworkCar, skybox::Skybox}, network::server_connection::{ServerConnection, NetworkEvent}};
use glow::*;
use sdl2::{
    image::ImageRWops,
    video::{GLContext, Window},
    EventPump, event::Event, keyboard::Keycode, pixels::PixelFormatEnum,
};

use super::{constants::{W_WIDTH, W_HEIGHT}, game_object::GameObject, matrices, skybox_shader::SkyboxShader};

const SHOW_FPS: bool = false;

pub struct Game<'a> {
    gl: &'a Context,
    window: &'a Window,
    events_loop: &'a mut EventPump,
    gl_context: &'a GLContext,
    pub shader: Shader3D<'a>,
    pub skybox_shader: SkyboxShader<'a>,
    pub model_matrix: RefCell<ModelMatrix>,
    pub view_matrix: RefCell<ViewMatrix>,
    pub projection_matrix: RefCell<ProjectionMatrix>,
    pub cube: Cube<'a>,
    last_time: Instant,
    pub game_objects: Vec<Box<RefCell<dyn GameObject<'a> + 'a>>>,
    pub objects_to_delete: RefCell<VecDeque<*const usize>>,
    pub delta_time: f32,
    pub server_connection: ServerConnection,
    pub frame_sum: i32,
    pub frame_time_sum: f32
}

impl<'a> Game<'a> {
    pub fn new(
        gl: &'a Context,
        window: &'a Window,
        events_loop: &'a mut EventPump,
        gl_context: &'a GLContext,
    ) -> Game<'a> {
        let shader = Shader3D::new(&gl);
        let cube = Cube::new(&gl);

        let model_matrix = matrices::ModelMatrix::new();
        let view_matrix = matrices::ViewMatrix::new();
        let mut projection_matrix = matrices::ProjectionMatrix::new();

        shader.use_shader();
        shader.set_view_matrix(view_matrix.get_matrix().as_slice());
        projection_matrix.set_perspective(60.0, W_WIDTH as f32 / W_HEIGHT as f32, 0.5, 500.0);
        shader.set_projection_matrix(projection_matrix.get_matrix().as_slice());

        Game {
            gl,
            window,
            events_loop,
            gl_context,
            shader,
            skybox_shader: SkyboxShader::new(gl),
            model_matrix: RefCell::new(model_matrix),
            view_matrix: RefCell::new(view_matrix),
            projection_matrix: RefCell::new(projection_matrix),
            cube,
            last_time: Instant::now(),
            delta_time: 0.0,
            game_objects: Vec::new(),
            objects_to_delete: RefCell::new(VecDeque::new()),
            server_connection: ServerConnection::new(),
            frame_sum: 0,
            frame_time_sum: 0.0,
        }
    }

    pub fn create_scene(&mut self) {
        self.server_connection.connect();

        self.add_game_object(Skybox::new(self.gl, self));
        self.add_game_object(Track::new(self.gl, self));
        self.add_game_object(Ground::new(self.gl, self));
        self.add_game_object(PlayerCar::new(self.gl, self));
        // self.add_game_object(FreecamController::new(self.gl));
    }

    #[inline(always)]
    fn add_game_object(&mut self, object: impl GameObject<'a> + 'a) {
        self.game_objects.push(Box::new(RefCell::new(object)) as Box<RefCell<dyn GameObject<'a>>>);
    }

    pub fn load_texture(&self, path_string: &str, repeat: bool) -> NativeTexture {
        let loader = sdl2::rwops::RWops::from_file(Path::new(path_string), "r").expect("Failed to load texture");
        let surface = loader.load_png().unwrap().convert_format(PixelFormatEnum::RGBA32).unwrap();
        let width = surface.width();
        let height = surface.height();

        unsafe {
            let tex_id = self.gl.create_texture().unwrap();
            self.gl.bind_texture(TEXTURE_2D, Some(tex_id));
            self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);

            if repeat {
                self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
                self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
            }

            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, width as i32, height as i32, 0, RGBA, UNSIGNED_BYTE, surface.without_lock());
            
            tex_id
        }
    }

    pub fn load_cubemap(&self, face_paths: &Vec<&str>) -> NativeTexture {
        let tex_id = unsafe {
            let tex_id = self.gl.create_texture().unwrap();
            self.gl.bind_texture(TEXTURE_CUBE_MAP, Some(tex_id));
            
            tex_id
        };

        for (i, face) in face_paths.iter().enumerate() {
            let loader = sdl2::rwops::RWops::from_file(Path::new(face), "r").expect("Failed to load texture for cube map");
            let surface = loader.load_png().unwrap().convert_format(PixelFormatEnum::RGBA32).unwrap();
            let width = surface.width();
            let height = surface.height();

            unsafe {
                self.gl.tex_image_2d(TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, RGB as i32, width as i32, height as i32, 0, RGB, UNSIGNED_BYTE, surface.without_lock());
            }
        }

        unsafe {
            self.gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MIN_FILTER, LINEAR as i32);
            self.gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MAG_FILTER, LINEAR as i32);
            self.gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            self.gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
            self.gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_R, CLAMP_TO_EDGE as i32);
        }

        tex_id
    }

    pub fn update(&mut self) {
        self.delta_time = (Instant::now() - self.last_time).as_secs_f32();
        self.last_time = Instant::now();

        if SHOW_FPS {
            if self.frame_time_sum >= 1.0 {
                println!("FPS: {}", self.frame_sum);
                self.frame_sum = 0;
                self.frame_time_sum = 0.0;
            }
            self.frame_time_sum += self.delta_time;
            self.frame_sum += 1;
        }

        self.server_connection.update();

        // Delete all game objects that were requested to be deleted
        {
            let mut objects_to_delete = self.objects_to_delete.borrow_mut();
            while !objects_to_delete.is_empty() {
                let object_to_delte = objects_to_delete.pop_back();

                match object_to_delte {
                    Some(object) => {
                        self.game_objects.retain(|o| o.as_ptr() as *const _ as *const usize != object);
                    }
                    None => ()
                }
            }
        }

        // Consume only the events that can be handled by the game struct
        loop {
            let event = self.server_connection.game_events.get_mut().back().map(|e| e.clone());
            
            match event {
                Some(NetworkEvent::PlayerConnected { player_id }) => {
                    self.add_game_object(NetworkCar::new(player_id, self.gl, self));
                    self.server_connection.game_events.get_mut().pop_back();
                }
                Some(NetworkEvent::PlayerDisconnected { .. }) | None => break
            }
        }

        for object in &self.game_objects {
            object.borrow_mut().update(self, self.gl);
        }
    }

    pub fn display(&mut self) {
        unsafe {
            self.gl.enable(DEPTH_TEST);
            self.gl.enable(BLEND);
            self.gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            self.gl.clear_color(0.03, 0.04, 0.13, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.viewport(0, 0, self.window.size().0 as i32, self.window.size().1 as i32);
        }

        let view_matrix = self.view_matrix.get_mut();

        self.shader.set_view_matrix(view_matrix.get_matrix().as_slice());
        self.shader.set_projection_matrix(self.projection_matrix.get_mut().get_matrix().as_slice());

        self.model_matrix.get_mut().load_identity();

        self.shader.set_light_position(&[0.0, 0.0, 8.0, 1.0]);
        self.shader.set_light_diffuse(&[0.5, 0.5, 0.5, 0.0]);
        self.shader.set_light_ambient(&[0.5, 0.5, 0.5, 0.0]);
        self.shader.set_light_specular(&[1.0, 1.0, 1.0, 0.0]);
        self.shader.set_eye_position(view_matrix.eye.x, view_matrix.eye.y, view_matrix.eye.z);

        for object in &self.game_objects {
            object.borrow().display(self, self.gl);
        }

        self.window.gl_swap_window();
    }

    pub fn main(&mut self) {
        let mut running = true;
        while running {
            {
                let events: Vec<Event> = self.events_loop.poll_iter().collect();

                for event in events {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                        _ => ()
                    }

                    for object in &self.game_objects {
                        object.borrow_mut().on_event(self, &event);
                    }
                }
            }

            self.update();
            self.display();
        }

        self.server_connection.end_connection();
    }
}
