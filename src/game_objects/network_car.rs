use glow::Context;
use nalgebra::Vector3;

use crate::{core::{game::Game, game_object::GameObject}, network::server_connection::NetworkEvent};

use super::car::Car;


pub struct NetworkCar<'a> {
    player_id: u8,
    car: Car<'a>,
}

impl<'a> NetworkCar<'a> {
    pub fn new (player_id: u8, gl: &'a Context, game: &Game) -> NetworkCar<'a> {
        let mut car = Car::new(gl, game);
        car.set_position(Vector3::new(5.0, 35.0, 0.0));

        NetworkCar { player_id, car }
    }
}

impl<'a> GameObject<'a> for NetworkCar<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        self.car.update(game, gl);

        let status = match game.server_connection.last_status(self.player_id) {
            Some(s) => s,
            None => return
        };

        loop {
            let mut game_events = game.server_connection.game_events.borrow_mut();
            let event = game_events.back();
            
            match event {
                Some(NetworkEvent::PlayerDisconnected { player_id }) if self.player_id == *player_id  => {
                    // Delete this car if the player has disconnected
                    game.objects_to_delete.borrow_mut().push_back(self as *const _ as *const usize);
                    game_events.pop_back();
                }
                _ => break
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