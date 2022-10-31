use std::{collections::{VecDeque, HashMap, HashSet}, net::UdpSocket, cell::RefCell};

use nalgebra::Vector3;

use super::{packets, parser::parse_packet};

enum Connection {
    Connected(UdpSocket),
    NotConnected,
}

#[derive(Clone)]
pub enum NetworkEvent {
    PlayerConnected { player_id: u8 },
    PlayerDisconnected { player_id: u8 },
    MoveToStartPos,
}

pub struct ServerConnection {
    connection: Connection,
    connected_players: HashSet<u8>,
    last_status: HashMap<u8, packets::StatusUpdate>,
    player_id: Option<u8>,
    pub game_events: RefCell<VecDeque<NetworkEvent>>,
}

impl ServerConnection {
    pub fn new() -> ServerConnection {
        ServerConnection {
            connection: Connection::NotConnected,
            connected_players: HashSet::new(),
            last_status: HashMap::new(),
            player_id: None,
            game_events: RefCell::new(VecDeque::new()),
        }
    }

    pub fn connect(&mut self, server_address: &str) {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to open socket for multiplayer");
        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking");

        socket
            .connect(server_address)
            .expect("Failed to connect to server");

        socket
            .send(&packets::GamePacket::Register.to_binary_data())
            .expect("Failed to send packet to register");

        self.connection = Connection::Connected(socket);
    }

    pub fn is_multiplayer(&self) -> bool {
        matches!(self.connection, Connection::Connected(_))
    }

    pub fn send_status_update(&self, position: &Vector3<f32>, rotation: f32, steering_angle: f32) {
        let player_id = match self.player_id {
            Some(player_id) => player_id,
            // Don't send any status updates if we havent gotten a player id yet
            None => return
        };

        let status = packets::StatusUpdate::new(player_id, packets::Vector3::from_nvector3(position), rotation, steering_angle);
        self.send_packet(packets::GamePacket::StatusUpdate(status));
    }

    pub fn end_connection(&mut self) {
        if let Some(player_id) = self.player_id {
            self.send_packet(packets::GamePacket::End { player_id });
        }

        self.connection = Connection::NotConnected;
    }

    fn send_packet(&self, packet: packets::GamePacket) {
        let socket = match &self.connection {
            Connection::Connected(s) => s,
            Connection::NotConnected => return,
        };

        match socket.send(&packet.to_binary_data()) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to send packet to server. {e}");
            }
        }
    }

    pub fn last_status(&self, player_id: u8) -> Option<&packets::StatusUpdate> {
        self.last_status.get(&player_id)
    }

    pub fn player_id(&self) -> Option<u8> {
        self.player_id
    }

    pub fn update(&mut self) {
        let socket = match &self.connection {
            Connection::Connected(s) => s,
            Connection::NotConnected => return,
        };

        let mut buffer = [0u8; 3000];
        loop {
            let size = match socket.recv(&mut buffer) {
                Ok(s) => s,
                Err(_) => return,
            };

            let packet = match parse_packet(&buffer[0..size]) {
                Ok(p) => p,
                Err(e) => {
                    println!("Recieved invalid packet. {e}");
                    continue;
                }
            };

            use packets::GamePacket::*;
            match packet {
                // We should never receive a register packet
                Register { .. } => (),
                NewPlayer { player_id } => {
                    self.connected_players.insert(player_id);
                    self.game_events.get_mut().push_back(NetworkEvent::PlayerConnected { player_id });
                }
                Inform { player_id } => {
                    self.player_id = Some(player_id);
                    self.game_events.get_mut().push_back(NetworkEvent::MoveToStartPos);
                    println!("Playing as player {player_id}");
                }
                StatusUpdate(status) => {
                    match self.last_status.get_mut(&status.player_id) {
                        Some(last_status) => {
                            *last_status = status;
                        }
                        None if self.connected_players.contains(&status.player_id) => {
                            self.last_status.insert(status.player_id, status);
                        }
                        None => ()
                    };
                }
                DropPlayer { player_id } => {
                    self.connected_players.remove(&player_id);
                    self.game_events.get_mut().push_back(NetworkEvent::PlayerDisconnected { player_id });
                }
                End { .. } => (),
            }
        }
    }
}
