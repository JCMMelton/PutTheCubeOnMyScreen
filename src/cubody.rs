
use cube::*;
use nphysics3d::object::{BodyHandle};

pub struct Cubody {
    pub cube: Cube,
    pub handle: BodyHandle
}

impl Cubody {
    pub fn new(cube: Cube, handle: BodyHandle) -> Self {
        Cubody {
            cube,
            handle
        }
    }
}