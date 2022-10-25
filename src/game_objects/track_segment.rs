use glow::{Context, NativeTexture};
use nalgebra::{Vector2, Vector3, Vector4};

use crate::{
    core::{
        color::Color,
        game::Game,
        game_object::{CollisionInfo, GameObject},
    },
    objects::{
        textured_square::TexturedSquare,
        track_corner::{TrackCorner, TrackCornerType},
    },
    utils::FacingDirection, game_objects::track_side::{Side, self},
};

use super::track_side::TrackSide;

const TRACK_ELEVATION: f32 = 30.0;
const TRACK_BOX_HEIGHT: f32 = 5.0;
const TRACK_WIDTH: f32 = 20.0;

pub enum SegmentType {
    // Position, direction, length
    Straight(Vector3<f32>, FacingDirection, f32),
    // Position
    RightCorner(Vector3<f32>),
    // Position, rotation
    UCorner(Vector3<f32>, f32),
}

enum SegmentObject<'a> {
    Straight(TexturedSquare<'a>, Vector3<f32>, FacingDirection, f32),
    RightCorner(TrackCorner<'a>, Vector3<f32>),
    UCorner(TrackCorner<'a>, Vector3<f32>, f32),
}

enum SegmentObjectCollisionInfo {
    Straight(f32, f32, f32, f32, f32, f32),
    RightCorner(f32, f32, f32, f32, f32, f32),
    UCorner(f32, f32, f32, f32, f32, f32),
}

pub struct TrackSegment<'a> {
    segment_object: SegmentObject<'a>,
    road_texture: NativeTexture,
    left_side: TrackSide,
    right_side: TrackSide
}

impl<'a> TrackSegment<'a> {
    pub fn new(segment_type: SegmentType, gl: &'a Context, game: &Game) -> TrackSegment<'a> {
        let road_texture = game.load_texture("./models/textures/road.png", true);

        use SegmentType::*;
        let segment_object = match &segment_type {
            Straight(pos, direction, length) => SegmentObject::Straight(
                TexturedSquare::new(gl, TRACK_WIDTH, *length, FacingDirection::North),
                pos.clone(),
                direction.clone(),
                *length,
            ),
            RightCorner(pos) => {
                SegmentObject::RightCorner(TrackCorner::new(gl, TrackCornerType::Right), pos.clone())
            }
            UCorner(pos, rot) => {
                SegmentObject::UCorner(TrackCorner::new(gl, TrackCornerType::UTurn), pos.clone(), *rot)
            }
        };

        let (right_side, left_side) = match segment_type {
            RightCorner(pos) => {
                let pos = pos + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);

                (TrackSide::new(pos, 0.0, 20.0, Side::Right, track_side::TrackSegmentSideType::RightCorner), TrackSide::new(pos, 0.0, 20.0, Side::Left, track_side::TrackSegmentSideType::RightCorner))
            }
            UCorner(pos, rot) => {
                let pos = pos + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);

                (TrackSide::new(pos, rot, 20.0, Side::Right, track_side::TrackSegmentSideType::UTurn), TrackSide::new(pos, rot, 20.0, Side::Left, track_side::TrackSegmentSideType::UTurn))
            }
            Straight(pos, direction, length) => {
                use FacingDirection::*;
                let rot = match direction {
                    North => 0.0,
                    West => 90f32.to_radians()
                };

                let pos = pos + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);

                (TrackSide::new(pos, rot, length, Side::Right, track_side::TrackSegmentSideType::Straight), TrackSide::new(pos, rot, length, Side::Left, track_side::TrackSegmentSideType::Straight))
            }
        };

        TrackSegment {
            road_texture,
            segment_object,
            left_side,
            right_side,
        }
    }
}

impl<'a> GameObject<'a> for TrackSegment<'a> {
    fn collision_info(&self) -> CollisionInfo {
        CollisionInfo::YCollision(TRACK_ELEVATION)
    }

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

        use SegmentObject::*;
        match &self.segment_object {
            Straight(object, pos, dir, length) => {
                model_matrix.push_stack();

                use FacingDirection::*;
                match dir {
                    North => {
                        model_matrix.add_translate(pos.x, pos.y + TRACK_ELEVATION + 0.1, pos.z);
                        model_matrix.add_scale(1.0, 1.0, 1.0);
                    }
                    West => {
                        model_matrix.add_translate(pos.x, pos.y + TRACK_ELEVATION + 0.1, pos.z);
                        model_matrix.add_scale(1.0, 1.0, 1.0);
                        model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);
                    }
                }

                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
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
                    pos.x,
                    pos.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0),
                    pos.z,
                );
                if matches!(dir, FacingDirection::West) {
                    model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);
                }
                model_matrix.add_scale(TRACK_WIDTH + 5.0, TRACK_BOX_HEIGHT, *length);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }
            RightCorner(object, pos) => {
                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, TRACK_ELEVATION + 0.1, pos.z);
                model_matrix.add_scale(TRACK_WIDTH * 10.0, 1.0, TRACK_WIDTH * 10.0);
                model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
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
                    pos.x,
                    (pos.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0)) - 0.02,
                    pos.z,
                );
                model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
                model_matrix.add_translate(-40.0, 0.0, -40.0);
                model_matrix.add_scale(120.0, TRACK_BOX_HEIGHT, 120.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }
            UCorner(object, pos, rot) => {
                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, TRACK_ELEVATION + 0.1, pos.z);
                model_matrix.add_scale(TRACK_WIDTH * 10.0, 1.0, TRACK_WIDTH * 10.0);
                model_matrix.add_rotation(0.0, *rot, 0.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
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
                    pos.x,
                    (pos.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0)) - 0.01,
                    pos.z,
                );
                model_matrix.add_rotation(0.0, *rot, 0.0);
                model_matrix.add_translate(0.0, 0.0, 30.0);
                model_matrix.add_scale(80.0, TRACK_BOX_HEIGHT, 70.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }
        }
    }
}
