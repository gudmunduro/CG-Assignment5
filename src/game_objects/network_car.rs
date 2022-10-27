use glow::Context;
use nalgebra::Vector3;

use crate::{core::{game::Game, game_object::GameObject}};

use super::car::Car;


pub struct NetworkCar<'a> {
    car: Car<'a>,
}

impl<'a> NetworkCar<'a> {
    pub fn new (gl: &'a Context, game: &Game) -> NetworkCar<'a> {
        let mut car = Car::new(gl, game);
        car.set_position(Vector3::new(5.0, 35.0, 0.0));

        NetworkCar { car }
    }
}

impl<'a> GameObject<'a> for NetworkCar<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        self.car.update(game, gl);

        let status = match game.server_connection.last_status() {
            Some(s) => s,
            None => return
        };

        self.car.set_position(status.position.into());
        self.car.set_angle(status.rotation);
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        if matches!(game.server_connection.last_status(), Some(_)) {
            self.car.display(game, gl);
        }
    }
}