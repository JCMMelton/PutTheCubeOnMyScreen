#[macro_use]
extern crate glium;
extern crate piston;
extern crate shader_version;
extern crate glutin_window;
extern crate gl;
extern crate nalgebra as na;


mod vertex;
mod cube;

use glium::*;
use na::{Matrix4, geometry, Vector3, Isometry3, Point3, Perspective3};
use std::f32;
use vertex::Vertex;
use cube::*;


fn main() {
    use glium::{glutin, Surface};
    let mut dimensions: [f32; 2] = [800.0, 600.0];
    let mut event_loop = glutin::EventsLoop::new();
    let mut window = glutin::WindowBuilder::new();
    window.window.dimensions = Some(glutin::dpi::LogicalSize::new(dimensions[0] as f64, dimensions[1] as f64));
    let context = glutin::ContextBuilder::new();
    let display: Display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut cubes: Vec<Cube> = Vec::new();
    cubes.push(Cube::new(
        Vector3::new(0.0, 10.0, -20.0),
        &display
    ));
    cubes.push(Cube::new(
        Vector3::new(10.0, 0.0, -20.0),
        &display
    ));

    let shape: Vec<Vertex> = cube_verts();

    let blank_buffer: [f32; 128] = [0.0; 128];
    let color_buffer = glium::buffer::Buffer::new(&display, &blank_buffer, glium::buffer::BufferType::UniformBuffer, glium::buffer::BufferMode::Dynamic).unwrap();
   color_buffer.write(&cube_colors());
    let vertex_buffer: VertexBuffer<Vertex> = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src   = include_str!("../assets/basic.vert");
    let fragment_shader_src = include_str!("../assets/basic.frag");
    let program: Program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut rotation = geometry::Rotation::from_matrix_unchecked( get_identity_matrix() );
    let perspective3 = geometry::Perspective3::new(dimensions[0]/dimensions[1], f32::consts::PI/2.0, 1.0, 1000.0);
    println!("{:?}", perspective3);

    let translation = geometry::Translation3::new(0.0, 0.0, -20.0);
    println!("{:?}", translation);

    let mut closed = false;
    let mut d: f32 = 0.001;

    let eye  = Point3::new(0.0, 0.0, 1.0);
    let targ = Point3::new(0.0, 0.0, 0.0);

    while !closed {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for cube in cubes.iter() {
            let model: Matrix4<f32> = Isometry3::<f32>::new(Vector3::x(), na::zero()).to_homogeneous() * cube.get_location_transform().to_homogeneous() * rotation;
            let view:  Matrix4<f32> = Isometry3::look_at_rh(&eye, &targ, &Vector3::y()).to_homogeneous();
            let projection: Perspective3<f32> = perspective3;
            let mvp = projection.as_matrix() * view * model;
            let transform: [[f32; 4]; 4] = na4_to_gl4(&mvp);
            let uniforms = uniform!{
                window_size: dimensions,
                transform:   transform,
                colors:      &color_buffer
            };
            target.draw(cube.get_vert_buffer(), &indices, &program, &uniforms, &Default::default()).unwrap();
        }
        target.finish().unwrap();

        event_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::Resized(size) => {
                        dimensions[0] = size.width  as f32;
                        dimensions[1] = size.height as f32;
                    },
                    _ => ()
                },
                _ => (),
            }
        });
        d += 0.01;
        rotation = geometry::Rotation::from_matrix_unchecked(get_z_rot(d) * get_x_rot(d) * get_y_rot(d) );
    }

}

fn get_x_rot(d: f32) -> Matrix4<f32> {
    let cd = f32::cos(d);
    let sd = f32::sin(d);
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0,  cd, -sd, 0.0,
        0.0,  sd,  cd, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn get_y_rot(d: f32) -> Matrix4<f32> {
    let cd = f32::cos(d);
    let sd = f32::sin(d);
    Matrix4::new(
         cd, 0.0,  sd, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sd, 0.0,  cd, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn get_z_rot(d: f32) -> Matrix4<f32> {
    let cd = f32::cos(d);
    let sd = f32::sin(d);
    Matrix4::new(
         cd, -sd, 0.0, 0.0,
         sd,  cd, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn get_identity_matrix() -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
 
fn na4_to_gl4(mat: &Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[0],  mat[1],  mat[2],  mat[3]],
        [mat[4],  mat[5],  mat[6],  mat[7]],
        [mat[8],  mat[9],  mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ]
}

fn cube_verts() -> Vec<Vertex> {
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

fn cube_colors() -> [f32; 128] {
    [
        1.0, 0.5, 0.5,
        0.5, 0.1, 0.5,
        0.5, 0.5, 1.0,

        0.5, 1.0, 0.5,
        1.0, 0.5, 0.5,
        0.5, 1.0, 0.5,

        0.5, 0.5, 1.0,
        0.5, 1.0, 0.5,
        1.0, 0.5, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 1.0, 0.5,
        0.5, 0.5, 1.0,
        0.0, 1.0, 0.5,

        0.5, 0.5
    ]
}