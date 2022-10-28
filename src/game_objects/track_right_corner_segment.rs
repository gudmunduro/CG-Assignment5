use glow::{Context, NativeTexture};
use nalgebra::Vector3;

use crate::{
    core::{color::Color, game::Game, game_object::GameObject},
    objects::track_corner::{TrackCorner, TrackCornerType},
};

use super::{
    track_segment::{TRACK_BOX_HEIGHT, TRACK_ELEVATION, TRACK_WIDTH},
    track_side::{self, Side, TrackSide},
};

pub struct TrackRightCornerSegment<'a> {
    road_texture: NativeTexture,
    segemnt_object: TrackCorner<'a>,
    position: Vector3<f32>,
    left_side: TrackSide,
    right_side: TrackSide,
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
        let (right_side, left_side) = (
            TrackSide::new(
                pos,
                0.0,
                20.0,
                Side::Right,
                track_side::TrackSegmentSideType::RightCorner,
            ),
            TrackSide::new(
                pos,
                0.0,
                20.0,
                Side::Left,
                track_side::TrackSegmentSideType::RightCorner,
            ),
        );

        TrackRightCornerSegment {
            road_texture,
            segemnt_object,
            position,
            left_side,
            right_side,
        }
    }
}

impl<'a> GameObject<'a> for TrackRightCornerSegment<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        self.left_side.display(game, gl);
        self.right_side.display(game, gl);

        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(&Color::new(0.5, 0.5, 0.5));
        game.shader.set_material_diffuse(&Color::new(0.7, 0.7, 0.7));
        game.shader
            .set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();
        model_matrix.add_translate(self.position.x, TRACK_ELEVATION + 0.1, self.position.z);
        model_matrix.add_scale(TRACK_WIDTH * 10.0, 1.0, TRACK_WIDTH * 10.0);
        model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.segemnt_object.draw(&game.shader, &self.road_texture);
        model_matrix.pop_stack();

        game.shader
            .set_material_ambient(&Color::new(0.89 / 2.0, 0.62 / 2.0, 0.14 / 2.0));
        game.shader
            .set_material_diffuse(&Color::new(0.89, 0.62, 0.14));
        game.shader
            .set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

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
