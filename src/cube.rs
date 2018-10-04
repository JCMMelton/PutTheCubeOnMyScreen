
extern crate glium;
extern crate nalgebra as na;

use glium::*;
use na::{Matrix4, Vector3, geometry, Isometry3};
use vertex::Vertex;

pub enum CubeType {
    Block,
    Light
}

pub struct Cube {
    cube_type: CubeType,
    scale: Matrix4<f32>,
    color: [f32; 3],
    size: f32,
    position: Isometry3<f32>
}

impl Cube {
    pub fn new(cube_type: CubeType, position: Isometry3<f32>, color: [f32; 3], size: f32) -> Self {
        let size = f32::min(size, 1.0);
        Cube {
            cube_type,
            color,
            size,
            scale : Matrix4::new(
                size, 0.0, 0.0, 0.0,
                0.0, size, 0.0, 0.0,
                0.0, 0.0, size, 0.0,
                0.0, 0.0, 0.0,  1.0
            ),
            position
        }
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn get_scale(&self) -> Matrix4<f32> {
        self.scale
    }

    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }

    pub fn get_x_pos(&self) -> f32 {
        self.position.translation.vector.x
    }

    pub fn get_y_pos(&self) -> f32 {
        self.position.translation.vector.y
    }

    pub fn get_z_pos(&self) -> f32 {
        self.position.translation.vector.z
    }

    pub fn get_type(&self) -> &CubeType {
        &self.cube_type
    }

    pub fn move_location(&mut self, new_pos: Isometry3<f32>) {
        self.position = new_pos;
    }

    pub fn get_model_transform(&self) -> Matrix4<f32> {
        self.position.to_homogeneous() * self.get_scale()
    }
}

fn get_normal(index: usize) -> (f32, f32, f32) {
    match index {
        0 => (0.0, 0.0, -1.0),
        1 => (0.0, 0.0, 1.0),

        2 => (-1.0, 0.0, 0.0),
        3 => (1.0, 0.0, 0.0),

        4 => (0.0, -1.0, 0.0),
        5 => (0.0, 1.0, 0.0),
        _ => (0.0, 0.0, 0.0)
    }
}

pub fn get_cube_verts(size: f32) -> Vec<Vertex> {
    let nz: f32 = -size;
    let pz: f32 =  size;
    let x:  f32 =  size;
    let y:  f32 =  size;

    vec![
        // back

        Vertex::new(-x, -y, nz, get_normal(0)),
        Vertex::new( x, -y, nz, get_normal(0)),
        Vertex::new( x,  y, nz, get_normal(0)),
        Vertex::new( x,  y, nz, get_normal(0)),
        Vertex::new(-x,  y, nz, get_normal(0)),
        Vertex::new(-x, -y, nz, get_normal(0)),

        // front

        Vertex::new(-x, -y, pz, get_normal(1)),
        Vertex::new( x, -y, pz, get_normal(1)),
        Vertex::new( x,  y, pz, get_normal(1)),
        Vertex::new( x,  y, pz, get_normal(1)),
        Vertex::new(-x,  y, pz, get_normal(1)),
        Vertex::new(-x, -y, pz, get_normal(1)),

        //right side

       Vertex::new(-x,  y, pz, get_normal(2)),
       Vertex::new(-x,  y, nz, get_normal(2)),
       Vertex::new(-x, -y, nz, get_normal(2)),
       Vertex::new(-x, -y, nz, get_normal(2)),
       Vertex::new(-x, -y, pz, get_normal(2)),
       Vertex::new(-x,  y, pz, get_normal(2)),

        // left side

       Vertex::new( x,  y, pz, get_normal(3)),
       Vertex::new( x,  y, nz, get_normal(3)),
       Vertex::new( x, -y, nz, get_normal(3)),
       Vertex::new( x, -y, nz, get_normal(3)),
       Vertex::new( x, -y, pz, get_normal(3)),
       Vertex::new( x,  y, pz, get_normal(3)),

        // top

        Vertex::new(-x, -y, nz, get_normal(4)),
        Vertex::new(-x, -y, pz, get_normal(4)),
        Vertex::new( x, -y, pz, get_normal(4)),
        Vertex::new( x, -y, pz, get_normal(4)),
        Vertex::new( x, -y, nz, get_normal(4)),
        Vertex::new(-x, -y, nz, get_normal(4)),

        // bottom

        Vertex::new(-x,  y, nz, get_normal(5)),
        Vertex::new(-x,  y, pz, get_normal(5)),
        Vertex::new( x,  y, pz, get_normal(5)),
        Vertex::new( x,  y, pz, get_normal(5)),
        Vertex::new( x,  y, nz, get_normal(5)),
        Vertex::new(-x,  y, nz, get_normal(5))
    ]
}
