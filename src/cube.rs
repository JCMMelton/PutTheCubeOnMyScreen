
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
    location: Vector3<f32>,
    rotation: Matrix4<f32>,
    vertex_buffer: VertexBuffer<Vertex>,
    translation: geometry::Translation3<f32>,
    color: [f32; 3]
}

impl Cube {
    pub fn new(cube_type: CubeType, location: Vector3<f32>, color: [f32; 3], display: &backend::Facade) -> Self {
        let shape: Vec<Vertex> = get_cube_verts();
        Cube {
            cube_type,
            location,
            color,
            rotation: Matrix4::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            vertex_buffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            translation: geometry::Translation3::from_vector(location)
        }
    }
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
            let cx = f32::cos(x);
            let sx = f32::sin(x);
            let cy = f32::cos(y);
            let sy = f32::sin(y);
            let cz = f32::cos(z);
            let sz = f32::sin(z);
            self.rotation = Matrix4::new(
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

    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }

    pub fn get_x_pos(&self) -> f32 {
        self.location.x
    }

    pub fn get_y_pos(&self) -> f32 {
        self.location.y
    }

    pub fn get_z_pos(&self) -> f32 {
        self.location.z
    }

    pub fn get_type(&self) -> &CubeType {
        &self.cube_type
    }

    pub fn move_location(&mut self, move_vector: Vector3<f32>) {
        self.location += move_vector;
        self.translation= geometry::Translation3::from_vector(self.location)
    }

    pub fn get_rotation(&self) -> &Matrix4<f32> {
        &self.rotation
    }

    pub fn get_location_transform(&self) -> &geometry::Translation3<f32> {
        &self.translation
    }

    pub fn get_vert_buffer(&self) -> &glium::VertexBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub fn get_model_transform(&self) -> Matrix4<f32> {
        Isometry3::<f32>::new(Vector3::x(), na::zero()).to_homogeneous() * self.get_location_transform().to_homogeneous() * self.get_rotation()
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

fn get_cube_verts() -> Vec<Vertex> {
    let nz: f32 = -1.0;
    let pz: f32 =  1.0;
    let x:  f32 =  1.0;
    let y:  f32 =  1.0;

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
