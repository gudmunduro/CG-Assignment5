use glow::{Context, NativeTexture};
use nalgebra::Vector3;

use crate::{
    core::{color::Color, game::Game, game_object::{GameObject, Collider}},
    objects::track_corner::{TrackCorner, TrackCornerType},
};

use super::{
    track_segment::{TRACK_BOX_HEIGHT, TRACK_ELEVATION, TRACK_WIDTH},
    track_side::{self, Side, TrackSide},
};

pub struct TrackUCornerSegment<'a> {
    road_texture: NativeTexture,
    segment_object: TrackCorner<'a>,
    position: Vector3<f32>,
    rotation: f32,
    sides: TrackSide,
}

impl<'a> TrackUCornerSegment<'a> {
    pub fn new(
        position: Vector3<f32>,
        rotation: f32,
        gl: &'a Context,
        game: &Game,
    ) -> TrackUCornerSegment<'a> {
        let road_texture = game.load_texture("./models/textures/road.png", true);
        let segment_object = TrackCorner::new(gl, TrackCornerType::UTurn);

        let pos = position + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);
        let sides =
            TrackSide::new(
                pos,
                rotation,
                20.0,
                track_side::TrackSegmentSideType::UTurn,
                game,
            );

        TrackUCornerSegment {
            road_texture,
            segment_object,
            position,
            rotation,
            sides,
        }
    }
}

impl<'a> GameObject<'a> for TrackUCornerSegment<'a> {
    fn collision_info(&self) -> Collider {
        self.sides.collision_info()
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {}

    fn update(&mut self, game: &Game, gl: &'a glow::Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        self.sides.display(game, gl);

        let mut model_matrix = game.model_matrix.borrow_mut();

        // Pavement
        game.shader.set_material_ambient(&Color::new(0.84 / 1.5, 0.73 / 1.5, 0.67 / 1.5));
        game.shader.set_material_diffuse(&Color::new(0.84, 0.73, 0.67));
        game.shader
            .set_material_specular(&Color::new(0.2, 0.2, 0.2));
        game.shader.set_shininess(100.0);

        model_matrix.push_stack();
        model_matrix.add_translate(self.position.x, TRACK_ELEVATION + 0.1, self.position.z);
        model_matrix.add_scale(TRACK_WIDTH * 10.0, 1.0, TRACK_WIDTH * 10.0);
        model_matrix.add_rotation(0.0, self.rotation, 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.segment_object.draw(&game.shader, &self.road_texture);
        model_matrix.pop_stack();

        // Platform
        game.shader
            .set_material_ambient(&Color::new(0.96 / 1.5, 0.58 / 1.5, 0.38 / 1.5));
        game.shader
            .set_material_diffuse(&Color::new(0.96, 0.58, 0.38));
        game.shader
            .set_material_specular(&Color::new(0.1, 0.1, 0.1));
        game.shader.set_shininess(100.0);

        model_matrix.push_stack();
        model_matrix.add_translate(
            self.position.x,
            (self.position.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0)) - 0.01,
            self.position.z,
        );
        model_matrix.add_rotation(0.0, self.rotation, 0.0);
        model_matrix.add_translate(0.0, 0.0, 30.0);
        model_matrix.add_scale(80.0, TRACK_BOX_HEIGHT, 70.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        game.cube.draw(&game.shader);
        model_matrix.pop_stack();
    }
}
