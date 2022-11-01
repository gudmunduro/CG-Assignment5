use std::rc::Rc;

use glow::Context;
use nalgebra::Vector3;

use crate::{
    core::{
        game::Game,
        game_object::{Collider, GameObject}, color::Color,
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

        let light_id = format!("NETWORK_CAR_{}", player_id);
        let mut lights = game.lights.borrow_mut();
        lights.add_light(&light_id);
        lights.set_light_diffuse(&light_id, &Color::new(0.89, 0.91, 1.00));
        lights
            .set_light_ambient(&light_id, &Color::with_alpha(0.0, 0.0, 0.0, 0.0));
        lights.set_light_specular(&light_id, &Color::new(0.89, 0.91, 1.00));
        lights.set_light_max_radius(&light_id, 50.0);

        NetworkCar {
            player_id,
            car,
            collider: None,
        }
    }

    fn light_id(&self) -> String {
        format!("NETWORK_CAR_{}", self.player_id)
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
                    game.lights.borrow_mut().remove_light(&self.light_id());

                    game_events.pop_front();
                }
                _ => break,
            }
        }

        let status = match game.server_connection.last_status(self.player_id) {
            Some(s) => s,
            None => return,
        };
        self.car.set_position(status.position.into());
        self.car.set_angle(status.rotation);
        self.car.set_steering_angle(status.steering_angle);

        let pos = self.car.light_position();
        game.lights.borrow_mut().set_light_position(&self.light_id(), &Vector3::new(pos.x, pos.y, pos.z));
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        // Only show the players car if we have recieved at least one status update from him
        if matches!(game.server_connection.last_status(self.player_id), Some(_)) {
            self.car.display(game, gl);
        }
    }
}
