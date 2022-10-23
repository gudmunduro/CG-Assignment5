pub mod car_sim;

#[derive(Clone)]
pub enum FacingDirection {
    North,
    West
}

pub fn limit(value: f32, from: f32, to: f32) -> f32 {
    value.min(to).max(from)
}