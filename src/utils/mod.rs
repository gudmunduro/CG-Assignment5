pub mod car_sim;

pub fn limit(value: f32, from: f32, to: f32) -> f32 {
    value.min(to).max(from)
}