use std::net::UdpSocket;

use nalgebra::Vector3;

use super::{models, parser::parse_packet};

enum Connection {
    Connected(UdpSocket),
    NotConnected
}

pub struct ServerConnection {
    connection: Connection,
    last_status: Option<models::StatusUpdate>
}

impl ServerConnection {
    pub fn new() -> ServerConnection {
        ServerConnection { connection: Connection::NotConnected, last_status: None }
    }

    pub fn connect(&mut self) {
        let socket = UdpSocket::bind("127.0.0.1:0")
            .expect("Failed to open socket for multiplayer");
        socket.set_nonblocking(true).expect("Failed to set socket to non-blocking");

        socket.connect("127.0.0.1:5899").expect("Failed to connect to server");

        socket.send(&models::RegisterPacket::new(0).binary_data())
            .expect("Failed to send packet to register");
        
        self.connection = Connection::Connected(socket);
    }

    pub fn is_multiplayer(&self) -> bool {
        matches!(self.connection, Connection::Connected(_))
    }

    pub fn send_status_update(&self, position: &Vector3<f32>, rotation: f32) {
        let socket = match &self.connection {
            Connection::Connected(s) => s,
            Connection::NotConnected => return
        };

        let status = models::StatusUpdate::new(models::Vector3::from_nvector3(position), rotation);
        match socket.send(&status.binary_data()) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to send status update to server. {e}");
            }
        }
    }

    pub fn last_status(&self) -> Option<&models::StatusUpdate> {
        self.last_status.as_ref()
    }

    pub fn update(&mut self) {
        let socket = match &self.connection {
            Connection::Connected(s) => s,
            Connection::NotConnected => return
        };

        let mut buffer = [0u8; 3000];
        loop {
            let size = match socket.recv(&mut buffer) {
                Ok(s) => s,
                Err(_) => return
            };

            let packet = match parse_packet(&buffer[0..size]) {
                Ok(p) => p,
                Err(e) => {
                    println!("Recieved invalid packet. {e}");
                    continue;
                }
            };

            use models::GamePacket::*;
            match packet {
                // We should never receive a register packet
                Register(_) => (),
                StatusUpdate(status) => {
                    self.last_status = Some(status);
                }
            }
        }
    }
}