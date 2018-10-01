use nphysics3d::solver::IntegrationParameters;
use nphysics3d::force_generator::ForceGenerator;
use nphysics3d::object::{BodyHandle, BodySet};
use nphysics3d::math::Velocity;
use nphysics3d::algebra::Force3;
use na::{Point3, Vector6};

use std::f32;
use cubody::*;

pub struct WorldForce {
    parts: Vec<BodyHandle>,
    timer: f32
}

impl WorldForce {
    pub fn new(parts: Vec<BodyHandle>) -> Self {
        WorldForce {
            parts,
            timer: 0.0
        }
    }
    pub fn add_body_part(&mut self, body: BodyHandle) {
        self.parts.push(body);
    }
}

impl ForceGenerator<f32> for WorldForce {
    fn apply(&mut self, _: &IntegrationParameters<f32>, bodies: &mut BodySet<f32>) -> bool {
        for handle in &self.parts {
            if bodies.contains(*handle) {
                let mut part = bodies.body_part_mut(*handle);
                let x = part.as_ref().position().translation.vector.x;
                let y = part.as_ref().position().translation.vector.y;
                let z = part.as_ref().position().translation.vector.z;
                let hxy = 0.1 + f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2));
                let hxz = 0.1 + f32::sqrt(f32::powi(x, 2) + f32::powi(z, 2));
                let hyz = 0.1 + f32::sqrt(f32::powi(y, 2) + f32::powi(z, 2));
                let force = &Force3::from_vector(
                    &Vector6::new(f32::sin(self.timer+hxy)/1.0, f32::sin(self.timer+hxz)/1.0, f32::sin(self.timer+hyz)/1.0, 0.0, 0.0, 0.0)
                );
                part.apply_force(&force);
            }
        }
        self.timer += 0.01;
        true
    }
}
pub struct Implosion {
    parts: Vec<BodyHandle>,
    center: Point3<f32>
}

impl Implosion {
    pub fn new(parts: Vec<BodyHandle>, center: Point3<f32>) -> Self {
        Implosion {
            parts,
            center
        }
    }
    pub fn add_body_part(&mut self, body: BodyHandle) {
        self.parts.push(body);
    }
}

impl ForceGenerator<f32> for Implosion {
    fn apply(&mut self, _: &IntegrationParameters<f32>, bodies: &mut BodySet<f32>) -> bool {
        for handle in &self.parts {
            if bodies.contains(*handle) {
                let mut part = bodies.body_part_mut(*handle);
                let delta_pos = part.as_ref().center_of_mass() - self.center;
                let force = Force3::linear(delta_pos * -0.2);
                part.apply_force(&force);
            }
        }
        true
    }
}


