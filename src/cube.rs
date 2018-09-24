
extern crate nalgebra as na;

use na::{Matrix4, geometry, Vector3, Isometry3, Point3, Perspective3};

pub struct Cube {
    location: Vector3,
    rotation: Matrix4,
}

impl Cube {
    pub fn new(location: Vector3) -> Self {
        Cube {
            location,
            rotation:     Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        }
    }
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
            let cx = f32::cos(x);
            let sx = f32::sin(x);
            let cy = f32::cos(y);
            let sy = f32::sin(y);
            let cz = f32::cos(z);
            let sz = f32::sin(z);
            self::rotation = Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0,  cx, -sx, 0.0,
                0.0,  sx,  cx, 0.0,
                0.0, 0.0, 0.0, 1.0
            ) *
            Matrix4::new(
                cy,  0.0,  sy, 0.0,
                0.0, 1.0, 0.0, 0.0,
                -sy, 0.0,  cy, 0.0,
                0.0, 0.0, 0.0, 1.0
            ) *
            Matrix4::new(
                cz, -sz,  0.0, 0.0,
                sz,  cz,  0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            )

    }
}