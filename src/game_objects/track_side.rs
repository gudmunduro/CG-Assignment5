use glow::Context;
use itertools::Itertools;
use nalgebra::{Vector3, Vector4};
use sdl2::event::Event;

use crate::core::{
    color::Color,
    game::Game,
    game_object::{Collider, GameObject},
};

pub enum Side {
    Left,
    Right,
}

pub enum TrackSegmentSideType {
    Straight,
    RightCorner,
    UTurn,
}

pub struct TrackSide {
    pos: Vector3<f32>,
    rot: f32,
    length: f32,
    side: Side,
    segment_type: TrackSegmentSideType,
    colliders: Vec<Collider>,
}

impl TrackSide {
    pub fn new(
        pos: Vector3<f32>,
        rot: f32,
        length: f32,
        side: Side,
        segment_type: TrackSegmentSideType,
        game: &Game,
    ) -> TrackSide {
        let colliders = if matches!(segment_type, TrackSegmentSideType::UTurn) {
            TrackSide::create_colliders(game, &pos, rot)
        } else {
            Vec::new()
        };

        TrackSide {
            pos,
            rot,
            length,
            side,
            segment_type,
            colliders,
        }
    }

    pub fn bezier_curve(
        p0: &Vector3<f32>,
        p1: &Vector3<f32>,
        p2: &Vector3<f32>,
        t: f32,
    ) -> Vector3<f32> {
        (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2
    }

    pub fn bezier_curve_3(
        p0: &Vector3<f32>,
        p1: &Vector3<f32>,
        p2: &Vector3<f32>,
        p3: &Vector3<f32>,
        t: f32,
    ) -> Vector3<f32> {
        (1.0 - t).powi(3) * p0
            + 3.0 * (1.0 - t).powi(2) * t * p1
            + 3.0 * (1.0 - t) * t.powi(2) * p2
            + t.powi(3) * p3
    }

    pub fn create_colliders(game: &Game, position: &Vector3<f32>, rotation: f32) -> Vec<Collider> {
        let mut model_matrix = game.model_matrix.borrow_mut();

        let (enter, control, exit) = (
            Vector3::new(-0.25, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.5),
            Vector3::new(0.25, 0.0, 0.0),
        );
        let (inner_enter, inner_control1, inner_control2, inner_exit) = (
            Vector3::new(-0.145, 0.0, 0.0),
            Vector3::new(-0.03, 0.0, 0.2),
            Vector3::new(0.03, 0.0, 0.2),
            Vector3::new(0.145, 0.0, 0.0),
        );

        let mut colliders = Vec::new();
        let points = 70;
        for (i1, i2) in (0..points).tuples() {
            let p1 = TrackSide::bezier_curve(&enter, &control, &exit, i1 as f32 / points as f32);
            let p2 = TrackSide::bezier_curve(&enter, &control, &exit, i2 as f32 / points as f32);
            let p = (0.5 * p1 + 0.5 * p2) * 200.0;
            let v = p1 - p2;
            let rot = f32::atan2(v.x, v.z);

            model_matrix.push_stack();

            model_matrix.add_translate(position.x, position.y, position.z);
            model_matrix.add_rotation(0.0, rotation, 0.0);

            // Outer
            model_matrix.push_stack();
            model_matrix.add_translate(p.x, 0.0, p.z);
            model_matrix.add_rotation(0.0, rot, 0.0);
            model_matrix.add_scale(1.0, 1.0, v.norm() * 400.0);
            
            let local_max = Vector4::new(0.5, 0.5, 0.5, 1.0);
            let local_min = Vector4::new(-0.5, -0.5, -0.5, 1.0);

            let max = model_matrix.matrix * local_max;
            let min = model_matrix.matrix * local_min;
            colliders.push(Collider::BoxCollider(
                min.x.min(max.x),
                min.y.min(max.y),
                min.z.min(max.z),
                max.x.max(min.x),
                max.y.max(min.y),
                max.z.max(min.z),
            ));

            model_matrix.pop_stack();

            // Inner
            let p1 = TrackSide::bezier_curve_3(
                &inner_enter,
                &inner_control1,
                &inner_control2,
                &inner_exit,
                i1 as f32 / points as f32,
            );
            let p2 = TrackSide::bezier_curve_3(
                &inner_enter,
                &inner_control1,
                &inner_control2,
                &inner_exit,
                i2 as f32 / points as f32,
            );
            let p = (0.5 * p1 + 0.5 * p2) * 200.0;
            let v = p1 - p2;
            let rot = f32::atan2(v.x, v.z);

            model_matrix.add_translate(p.x, 0.0, p.z);
            model_matrix.add_rotation(0.0, rot, 0.0);
            model_matrix.add_scale(1.0, 1.0, v.norm() * 400.0);

            let local_p0 = Vector4::new(0.5, 0.0, 0.0, 1.0);
            let local_p1 = Vector4::new(-0.5, 0.0, 0.0, 1.0);

            let p0 = model_matrix.matrix * local_p0;
            let p1 = model_matrix.matrix * local_p1;
            colliders.push(Collider::InfiniteYPlaneCollider(
                p0.xyz(),
                p1.xyz()
            ));

            model_matrix.pop_stack();
        }

        colliders
    }
}

impl<'a> GameObject<'a> for TrackSide {
    fn collision_info(&self) -> Collider {
        Collider::MultiCollider(self.colliders.clone())
    }

    fn on_event(&mut self, game: &Game, event: &Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        use TrackSegmentSideType::*;
        match self.segment_type {
            Straight => {
                for i in 0..(self.length as i32 / 4) {
                    model_matrix.push_stack();

                    let color = if i % 2 == 0 {
                        Color::new(1.0, 0.0, 0.0)
                    } else {
                        Color::new(1.0, 1.0, 1.0)
                    };

                    game.shader.set_material_ambient(&color);
                    game.shader.set_material_diffuse(&color);
                    game.shader
                        .set_material_specular(&Color::new(1.0, 1.0, 1.0));
                    game.shader.set_shininess(3.0);

                    use Side::*;
                    let offset = match self.side {
                        Left => 10.5,
                        Right => -10.5,
                    };

                    model_matrix.add_translate(self.pos.x, self.pos.y, self.pos.z);
                    model_matrix.add_rotation(0.0, self.rot, 0.0);
                    model_matrix.add_translate(0.0, 0.0, -(self.length / 2.0));
                    model_matrix.add_translate(offset, 0.0, 4.0 * i as f32);
                    model_matrix.add_scale(1.0, 1.0, 4.0);
                    game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                    game.cube.draw(&game.shader);

                    model_matrix.pop_stack();
                }
            }
            RightCorner => {
                let (enter, control, exit) = (
                    Vector3::new(-0.5, 0.0, 0.0),
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(0.0, 0.0, -0.5),
                );

                let points = 70;
                for (i1, i2) in (0..points).tuples() {
                    let p1 =
                        TrackSide::bezier_curve(&enter, &control, &exit, i1 as f32 / points as f32);
                    let p2 =
                        TrackSide::bezier_curve(&enter, &control, &exit, i2 as f32 / points as f32);
                    let p = (0.5 * p1 + 0.5 * p2) * 200.0;
                    let v = p1 - p2;
                    let rot = f32::atan2(v.x, v.z);

                    model_matrix.push_stack();

                    let color = if i1 % 4 == 0 {
                        Color::new(1.0, 0.0, 0.0)
                    } else {
                        Color::new(1.0, 1.0, 1.0)
                    };

                    game.shader.set_material_ambient(&color);
                    game.shader.set_material_diffuse(&color);
                    game.shader
                        .set_material_specular(&Color::new(1.0, 1.0, 1.0));
                    game.shader.set_shininess(3.0);

                    model_matrix.add_translate(self.pos.x, self.pos.y, self.pos.z);
                    model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
                    model_matrix.add_translate(p.x, 0.0, p.z);
                    model_matrix.add_rotation(0.0, rot, 0.0);

                    model_matrix.push_stack();
                    model_matrix.add_scale(1.0, 1.0, v.norm() * 400.0);
                    game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                    game.cube.draw(&game.shader);
                    model_matrix.pop_stack();

                    if i1 % 4 == 0 {
                        let color = if i1 % 8 == 0 {
                            Color::new(1.0, 0.0, 0.0)
                        } else {
                            Color::new(1.0, 1.0, 1.0)
                        };
                        game.shader.set_material_ambient(&color);
                        game.shader.set_material_diffuse(&color);

                        model_matrix.add_translate(-20.0, 0.0, 0.0);
                        model_matrix.add_scale(1.0, 1.0, v.norm() * 600.0);
                        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                        game.cube.draw(&game.shader);
                    }

                    model_matrix.pop_stack();
                }
            }
            UTurn => {
                let (enter, control, exit) = (
                    Vector3::new(-0.25, 0.0, 0.0),
                    Vector3::new(0.0, 0.0, 0.5),
                    Vector3::new(0.25, 0.0, 0.0),
                );
                let (inner_enter, inner_control1, inner_control2, inner_exit) = (
                    Vector3::new(-0.145, 0.0, 0.0),
                    Vector3::new(-0.03, 0.0, 0.2),
                    Vector3::new(0.03, 0.0, 0.2),
                    Vector3::new(0.145, 0.0, 0.0),
                );

                let points = 70;
                for (i1, i2) in (0..points).tuples() {
                    let p1 =
                        TrackSide::bezier_curve(&enter, &control, &exit, i1 as f32 / points as f32);
                    let p2 =
                        TrackSide::bezier_curve(&enter, &control, &exit, i2 as f32 / points as f32);
                    let p = (0.5 * p1 + 0.5 * p2) * 200.0;
                    let v = p1 - p2;
                    let rot = f32::atan2(v.x, v.z);

                    model_matrix.push_stack();

                    let color = if i1 % 4 == 0 {
                        Color::new(1.0, 0.0, 0.0)
                    } else {
                        Color::new(1.0, 1.0, 1.0)
                    };

                    game.shader.set_material_ambient(&color);
                    game.shader.set_material_diffuse(&color);
                    game.shader
                        .set_material_specular(&Color::new(1.0, 1.0, 1.0));
                    game.shader.set_shininess(3.0);

                    model_matrix.add_translate(self.pos.x, self.pos.y, self.pos.z);
                    model_matrix.add_rotation(0.0, self.rot, 0.0);

                    // Outer
                    model_matrix.push_stack();
                    model_matrix.add_translate(p.x, 0.0, p.z);
                    model_matrix.add_rotation(0.0, rot, 0.0);
                    model_matrix.add_scale(1.0, 1.0, v.norm() * 400.0);
                    game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                    game.cube.draw(&game.shader);
                    model_matrix.pop_stack();

                    // Inner
                    let p1 = TrackSide::bezier_curve_3(
                        &inner_enter,
                        &inner_control1,
                        &inner_control2,
                        &inner_exit,
                        i1 as f32 / points as f32,
                    );
                    let p2 = TrackSide::bezier_curve_3(
                        &inner_enter,
                        &inner_control1,
                        &inner_control2,
                        &inner_exit,
                        i2 as f32 / points as f32,
                    );
                    let p = (0.5 * p1 + 0.5 * p2) * 200.0;
                    let v = p1 - p2;
                    let rot = f32::atan2(v.x, v.z);

                    model_matrix.add_translate(p.x, 0.0, p.z);
                    model_matrix.add_rotation(0.0, rot, 0.0);
                    model_matrix.add_scale(1.0, 1.0, v.norm() * 400.0);
                    game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                    game.cube.draw(&game.shader);

                    model_matrix.pop_stack();
                }
            }
        }
    }
}
