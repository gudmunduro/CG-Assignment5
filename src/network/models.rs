#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_nvector3(vector: &nalgebra::Vector3<f32>) -> Vector3 {
        Vector3 { x: vector.x, y: vector.y, z: vector.z }
    }

    pub fn binary_data(&self) -> Vec<u8> {
        [self.x.to_le_bytes(), self.y.to_le_bytes(), self.z.to_le_bytes()].concat()
    }
}

impl From<Vector3> for nalgebra::Vector3<f32> {
    fn from(vector: Vector3) -> Self {
        Self::new(vector.x, vector.y, vector.z)
    }
}

pub struct StatusUpdate {
    pub position: Vector3,
    pub rotation: f32
}

impl StatusUpdate {
    pub fn new(position: Vector3, rotation: f32) -> StatusUpdate {
        StatusUpdate { position, rotation }
    }

    pub fn binary_data(&self) -> Vec<u8> {
        [vec![1u8], self.position.binary_data(), self.rotation.to_le_bytes().to_vec()].concat()
    }
}

pub enum GamePacket {
    Register,
    Inform { player_id: u8 },
    StatusUpdate(StatusUpdate),
    DropPlayer { player_id: u8 },
    End { player_id: u8 },
}

impl GamePacket {
    pub fn binary_data(&self) -> Vec<u8> {
        use GamePacket::*;
        match self {
            Register => vec![0],
            Inform { player_id } => vec![5, *player_id],
            StatusUpdate(s) => s.binary_data(),
            DropPlayer { player_id } => vec![4, *player_id],
            End { player_id } => vec![3, *player_id]
        }
    }
}