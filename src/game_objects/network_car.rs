use std::rc::Rc;

use glow::Context;
use nalgebra::Vector3;

use crate::{
    core::{
        game::Game,
        game_object::{Collider, GameObject},
    },
    network::server_connection::NetworkEvent,
    objects::mesh_model::MeshModel,
};

use super::car::Car;

pub struct NetworkCar<'a> {
    player_id: u8,
    car: Car<'a>,
    collider: Option<Collider>,
}

impl<'a> NetworkCar<'a> {
    pub fn new(
        player_id: u8,
        car_model: Rc<MeshModel<'a>>,
        wheel_model: Rc<MeshModel<'a>>,
        gl: &'a Context,
        game: &Game,
    ) -> NetworkCar<'a> {
        let mut car = Car::new(false, car_model, wheel_model, gl, game);
        car.set_position(Vector3::new(5.0, 35.0, 0.0));

        NetworkCar {
            player_id,
            car,
            collider: None,
        }
    }
}

impl<'a> GameObject<'a> for NetworkCar<'a> {
    fn collision_info(&self) -> Collider {
        self.collider.clone().unwrap_or(Collider::NoCollision)
    }

    fn on_event(&mut self, _game: &Game, _event: &sdl2::event::Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {
        self.car.update(game, gl);
        let (min_x, min_y, min_z, max_x, max_y, max_z) = self.car.car_cube(game);
        self.collider = Some(Collider::BoxCollider(
            min_x, min_y, min_z, max_x, max_y, max_z,
        ));

        let status = match game.server_connection.last_status(self.player_id) {
            Some(s) => s,
            None => return,
        };

        loop {
            let mut game_events = game.server_connection.game_events.borrow_mut();
            let event = game_events.front();

            match event {
                Some(NetworkEvent::PlayerDisconnected { player_id })
                    if self.player_id == *player_id =>
                {
                    // Delete this car if the player has disconnected
                    game.objects_to_delete
                        .borrow_mut()
                        .push_back(self as *const _ as *const usize);
                    game_events.pop_front();
                }
                _ => break,
            }
        }

        self.car.set_position(status.position.into());
        self.car.set_angle(status.rotation);
        self.car.set_steering_angle(status.steering_angle);
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        if matches!(game.server_connection.last_status(self.player_id), Some(_)) {
            self.car.display(game, gl);
        }
    }
}
