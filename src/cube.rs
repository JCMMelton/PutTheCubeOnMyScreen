
extern crate glium;
extern crate nalgebra as na;

use glium::*;
use na::{Matrix4, Vector3, geometry};
use vertex::Vertex;

pub struct Cube {
    location: Vector3<f32>,
    rotation: Matrix4<f32>,
    vertex_buffer: VertexBuffer<Vertex>,
    translation: geometry::Translation3<f32>
}

impl Cube {
    pub fn new(location: Vector3<f32>, display: &backend::Facade) -> Self {
        let shape: Vec<Vertex> = get_cube_verts();
        Cube {
            location,
            rotation:     Matrix4::new(
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

    pub fn get_location_transform(&self) -> &geometry::Translation3<f32> {
        &self.translation
    }

    pub fn get_vert_buffer(&self) -> &glium::VertexBuffer<Vertex> {
        &self.vertex_buffer
    }
}

fn get_cube_verts() -> Vec<Vertex> {
    let nz: f32 = -1.0;
    let pz: f32 =  1.0;
    let x:  f32 =  1.0;
    let y:  f32 =  1.0;
    vec![
        //1
        Vertex::new(-x, -y, nz),
        Vertex::new(-x, -y, pz),
        Vertex::new(-x,  y, pz),

        //2
        Vertex::new( x,  y, nz),
        Vertex::new(-x, -y, nz),
        Vertex::new(-x,  y, nz),

        //3
        Vertex::new( x, -y, pz),
        Vertex::new(-x, -y, nz),
        Vertex::new( x, -y, nz),

        //4
        Vertex::new( x,  y, nz),
        Vertex::new( x, -y, nz),
        Vertex::new(-x, -y, nz),

        //5
        Vertex::new(-x, -y, nz),
        Vertex::new(-x,  y, pz),
        Vertex::new(-x,  y, nz),

        //6
        Vertex::new( x, -y, pz),
        Vertex::new(-x, -y, pz),
        Vertex::new(-x, -y, nz),

        //7
        Vertex::new(-x,  y, pz),
        Vertex::new(-x, -y, pz),
        Vertex::new( x, -y, pz),

        //8
        Vertex::new(x,  y, pz),
        Vertex::new(x, -y, nz),
        Vertex::new(x,  y, nz),

        //9
        Vertex::new(x, -y, nz),
        Vertex::new(x,  y, pz),
        Vertex::new(x, -y, pz),

        //10
        Vertex::new( x, y, pz),
        Vertex::new( x, y, nz),
        Vertex::new(-x, y, nz),

        //11
        Vertex::new( x, y, pz),
        Vertex::new(-x, y, nz),
        Vertex::new(-x, y, pz),

        //12
        Vertex::new( x,  y, pz),
        Vertex::new(-x,  y, pz),
        Vertex::new( x, -y, pz)
    ]
}