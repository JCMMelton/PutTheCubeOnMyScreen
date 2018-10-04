extern crate nalgebra as na;

use na::{Matrix4, Isometry3};
use vertex::Vertex;

pub struct Sheet {
    scale: Matrix4<f32>,
    color: [f32; 3],
    position: Isometry3<f32>
}

impl Sheet {
    pub fn new(position: Isometry3<f32>, color: [f32; 3], size: f32) -> Self {
        Sheet {
            color,
            scale : Matrix4::new(
                size, 0.0, 0.0, 0.0,
                0.0, size, 0.0, 0.0,
                0.0, 0.0, size, 0.0,
                0.0, 0.0, 0.0,  1.0
            ),
            position
        }
    }

    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }

    pub fn get_scale(&self) -> Matrix4<f32> {
        self.scale
    }

    pub fn get_model_transform(&self) -> Matrix4<f32> {
        self.position.to_homogeneous() * self.get_scale()
    }
}

pub fn get_sheet_verts(res: usize) -> Vec<Vertex> {
    let mut shape: Vec<Vertex> = Vec::new();
    let hres = ((res as f32)/2.0) as i32;
    let resf: f32 = res as f32;
    let unit: f32 = 1.0/resf;
    for h in -hres..hres {
        let hf: f32 = h as f32;
        for w in -hres..hres {
            let wf: f32 = w  as f32;
            let x: f32 = wf * unit;
            let y: f32 = hf * unit;
            shape.push(Vertex::new(x, y, 0.0, (0.0, 0.0, 1.0)));
            shape.push(Vertex::new(x+unit, y, 0.0, (0.0, 0.0, 1.0)));
            shape.push(Vertex::new(x+unit, y+unit, 0.0, (0.0, 0.0, 1.0)));
            shape.push(Vertex::new(x+unit, y+unit, 0.0, (0.0, 0.0, 1.0)));
            shape.push(Vertex::new(x, y+unit, 0.0, (0.0, 0.0, 1.0)));
            shape.push(Vertex::new(x, y, 0.0, (0.0, 0.0, 1.0)));
        }
    }
    shape
}