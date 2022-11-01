use std::f32;

use nalgebra::Vector3;

use crate::utils::limit;

const DRAG_FORCE: f32 = 5.0;
const ROLLING_RESISTANCE: f32 = 30.0;
const CA_F: f32 = -5.0;
const CA_R: f32 = -5.20;
const MAX_GRIP: f32 = 2.0;

const CAR_B: f32 = 1.0;
const CAR_C: f32 = 1.0;
const CAR_WHEELBASE: f32 = CAR_B * CAR_C;
#[allow(dead_code)]
const CAR_H: f32 = 1.0;
const CAR_MASS: f32 = 600.0;
const CAR_INERTIA: f32 = 600.0;
#[allow(dead_code)]
const CAR_WIDTH: f32 = 1.5;
#[allow(dead_code)]
const CAR_LENGTH: f32 = 3.0;
const CAR_WHEEL_LENGTH: f32 = 1.4;
#[allow(dead_code)]
const CAR_WHEEL_WIDTH: f32 = 0.3;

const GRAVITY: f32 = 9.8;

#[derive(Clone)]
pub struct CarState {
    pub position_wc: Vector3<f32>,
    pub velocity_wc: Vector3<f32>,
    pub angle: f32,
    pub angular_velocity: f32,
    pub steering_angle: f32,
    pub throttle: f32,
    pub brake: f32,
    pub wheel_rotation_speed: f32,
}

impl CarState {
    pub fn new() -> CarState {
        CarState {
            position_wc: Vector3::zeros(),
            velocity_wc: Vector3::zeros(),
            angle: 0.0,
            angular_velocity: 0.0,
            steering_angle: 0.0,
            throttle: 0.0,
            brake: 0.0,
            wheel_rotation_speed: 0.0,
        }
    }

    pub fn perform_physics_time_step(
        &mut self,
        delta_time: f32,
        front_slip: bool,
        rear_slip: bool,
    ) {
        let sin_ang = self.angle.sin();
        let cos_ang = self.angle.cos();

        let velocity = Vector3::new(
            cos_ang * self.velocity_wc.z + sin_ang * self.velocity_wc.x,
            0.0,
            -sin_ang * self.velocity_wc.z + cos_ang * self.velocity_wc.x,
        );

        // Calculate lateral force
        let yaw_speed = CAR_WHEELBASE * 0.5 * self.angular_velocity;

        let rot_angle = if velocity.x == 0.0 {
            0.0
        } else {
            f32::atan2(yaw_speed, velocity.x)
        };

        let sideslip = if velocity.x == 0.0 {
            0.0
        } else {
            f32::atan2(velocity.z, velocity.x)
        };

        let slip_angle_front = sideslip + rot_angle - self.steering_angle;
        let slip_angle_rear = sideslip - rot_angle;

        let weight = CAR_MASS * GRAVITY * 0.5;

        // Lateral force on the front wheels
        let mut front_lateral_force = Vector3::new(
            0.0,
            0.0,
            limit(CA_F * slip_angle_front, -MAX_GRIP, MAX_GRIP) * weight,
        );
        if front_slip {
            front_lateral_force.z *= 0.5;
        }

        // Lateral force on the rear wheels
        let mut rear_lateral_force = Vector3::new(
            0.0,
            0.0,
            limit(CA_R * slip_angle_rear, -MAX_GRIP, MAX_GRIP) * weight,
        );
        if rear_slip {
            rear_lateral_force.z *= 0.5;
        }

        let mut traction_force = Vector3::new(
            100.0 * (self.throttle - self.brake * velocity.x.signum()),
            0.0,
            0.0,
        );
        self.wheel_rotation_speed =
            ((traction_force.x * delta_time) / CAR_WHEEL_LENGTH) * 2.0 * f32::consts::PI;

        if rear_slip {
            traction_force.x *= 0.5;
        }

        // Force and torque on body

        let resistance = -Vector3::new(
            ROLLING_RESISTANCE * velocity.x + DRAG_FORCE * velocity.x * velocity.x.abs(),
            0.0,
            ROLLING_RESISTANCE * velocity.z + DRAG_FORCE * velocity.z * velocity.z.abs(),
        );

        let force = traction_force
            + Vector3::new(
                self.steering_angle.sin() * front_lateral_force.x,
                0.0,
                self.steering_angle.cos() * front_lateral_force.z,
            )
            + rear_lateral_force
            + resistance;

        let torque = CAR_B * front_lateral_force.z - CAR_C * rear_lateral_force.z;

        // Acceleration

        let acceleration = force / CAR_MASS;
        let angular_acceleration = torque / CAR_INERTIA;

        // Velocity and position
        let acceleration_wc = Vector3::new(
            cos_ang * acceleration.z + sin_ang * acceleration.x,
            0.0,
            -sin_ang * acceleration.z + cos_ang * acceleration.x,
        );

        self.velocity_wc.x += delta_time * acceleration_wc.x;
        self.velocity_wc.z += delta_time * acceleration_wc.z;

        self.position_wc.x += delta_time * self.velocity_wc.x;
        self.position_wc.z += delta_time * self.velocity_wc.z;

        // Angular velocity and heading
        self.angular_velocity += delta_time * angular_acceleration;
        self.angle += delta_time * self.angular_velocity;
    }

    pub fn peek_time_step(&self, delta_time: f32, front_slip: bool, rear_slip: bool) -> CarState {
        let mut future_state = self.clone();
        future_state.perform_physics_time_step(delta_time, front_slip, rear_slip);
        future_state
    }
}
