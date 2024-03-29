use glow::{Context, NativeTexture};
use nalgebra::Vector3;

use crate::{
    core::{
        color::Color,
        game::Game,
        game_object::{Collider, GameObject},
    },
    objects::track_corner::{TrackCorner, TrackCornerType},
};

use super::{
    track_segment::{TRACK_BOX_HEIGHT, TRACK_ELEVATION, TRACK_WIDTH},
    track_side::{TrackSide, self},
};

pub struct TrackRightCornerSegment<'a> {
    road_texture: NativeTexture,
    segemnt_object: TrackCorner<'a>,
    position: Vector3<f32>,
    sides: TrackSide,
}

impl<'a> TrackRightCornerSegment<'a> {
    pub fn new(
        position: Vector3<f32>,
        gl: &'a Context,
        game: &Game,
    ) -> TrackRightCornerSegment<'a> {
        let road_texture = game.load_texture("./models/textures/road.png", true);
        let segemnt_object = TrackCorner::new(gl, TrackCornerType::Right);

        let pos = position + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);
        let sides = TrackSide::new(
            pos,
            0.0,
            20.0,
            track_side::TrackSegmentSideType::RightCorner,
            game,
        );

        TrackRightCornerSegment {
            road_texture,
            segemnt_object,
            position,
            sides,
        }
    }
}

impl<'a> GameObject<'a> for TrackRightCornerSegment<'a> {
    fn collision_info(&self) -> Collider {
        self.sides.collision_info()
    }

    fn on_event(&mut self, _game: &Game, _event: &sdl2::event::Event) {}

    fn update(&mut self, _game: &Game, _gl: &'a Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        self.sides.display(game, gl);

        let mut model_matrix = game.model_matrix.borrow_mut();

        // Pavement
        game.shader
            .set_material_ambient(&Color::new(0.84 / 1.5, 0.73 / 1.5, 0.67 / 1.5));
        game.shader
            .set_material_diffuse(&Color::new(0.84, 0.73, 0.67));
        game.shader
            .set_material_specular(&Color::new(0.2, 0.2, 0.2));
        game.shader.set_shininess(100.0);

        model_matrix.push_stack();
        model_matrix.add_translate(self.position.x, TRACK_ELEVATION + 0.1, self.position.z);
        model_matrix.add_scale(TRACK_WIDTH * 10.0, 1.0, TRACK_WIDTH * 10.0);
        model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.segemnt_object.draw(&game.shader, &self.road_texture);
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
            (self.position.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0)) - 0.02,
            self.position.z,
        );
        model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
        model_matrix.add_translate(-40.0, 0.0, -40.0);
        model_matrix.add_scale(120.0, TRACK_BOX_HEIGHT, 120.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        game.cube.draw(&game.shader);
        model_matrix.pop_stack();
    }
}
