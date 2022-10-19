use nalgebra::Vector3;


const ENGINE_FORCE: f32 = 5000.0;
const DRAG_FORCE: f32 = 0.4257;
const ROLLING_RESISTANCE: f32 = 12.8;
const CAR_MASS: f32 = 800.0;

pub fn accelerate_time_step(car_pos: &Vector3<f32>, car_velocity: &Vector3<f32>, car_heading: &Vector3<f32>, delta_time: f32) -> (Vector3<f32>, Vector3<f32>) {
    let traction = car_heading * ENGINE_FORCE;
    let drag = -DRAG_FORCE * car_velocity * car_velocity.norm();
    let rolling_resitance = -ROLLING_RESISTANCE * car_velocity;
    let long_force = traction + drag + rolling_resitance;
    
    let acceleration = long_force / CAR_MASS;
    let velocity = car_velocity + delta_time * acceleration;
    let position = car_pos + delta_time * velocity;

    (position, velocity)
}