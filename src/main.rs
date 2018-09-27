#[macro_use]
extern crate glium;
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
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display: Display = glium::Display::new(window, context, &event_loop).unwrap();

    let light_position: (f32, f32, f32) = (-5.0, 0.0, 18.0);
    let mut cubes: Vec<Cube> = Vec::new();
    cubes.push(Cube::new(
        CubeType::Block,
        Vector3::new(5.0, 0.0, 10.0),
        &display
    ));
    cubes.push(Cube::new(
        CubeType::Light,
        Vector3::new(light_position.0, light_position.1, light_position.2),
        &display
    ));

    let blank_buffer: [f32; 128] = [0.0; 128];
    let color_buffer = glium::buffer::Buffer::new(&display, &blank_buffer, glium::buffer::BufferType::UniformBuffer, glium::buffer::BufferMode::Dynamic).unwrap();
   color_buffer.write(&cube_colors());
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let block_vertex_shader_src   = include_str!("../assets/block.vert");
    let block_fragment_shader_src = include_str!("../assets/block.frag");
    let block_program: Program = glium::Program::from_source(&display, block_vertex_shader_src, block_fragment_shader_src, None).unwrap();

    let light_vertex_shader_src   = include_str!("../assets/light.vert");
    let light_fragment_shader_src = include_str!("../assets/light.frag");
    let light_program: Program = glium::Program::from_source(&display, light_vertex_shader_src, light_fragment_shader_src, None).unwrap();

    let projection = geometry::Perspective3::new(dimensions[0]/dimensions[1], f32::consts::PI/2.0, 0.1, 1000.0);

    let mut closed = false;
    let mut d: f32 = 0.001;

    let targ = Point3::new(0.0, 0.0, 1.0);

    let light_color:  [f32; 3] = [1.0, 1.0, 1.0];
    let object_color: [f32; 3] = [1.0, 0.5, 0.3];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLessOrEqual,
           // test: glium::DepthTest::IfMoreOrEqual,
            write: true,
            .. Default::default()
        },
//         backface_culling: glium::BackfaceCullingMode::CullingDisabled,
//         backface_culling: glium::BackfaceCullingMode::CullClockwise,
         // backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };
    let mut rotate_cubes: bool = false;
    while !closed {

        let mut target = display.draw();
        target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

        let eye  = Point3::new(f32::cos(d)*0.1, f32::sin(d)*0.1, 0.0);
        let view:  Matrix4<f32> = Isometry3::look_at_rh(&eye, &targ, &Vector3::y()).to_homogeneous();

        for cube in cubes.iter_mut() {

            if rotate_cubes {
                match cube.get_type() {
                    CubeType::Block => {
                        cube.rotate(f32::cos(d), f32::sin(d), d);
                    },
                    _ => ()
                };
            }
            
            let uniforms = uniform!{
                window_size: dimensions,
                model:       na4_to_gl4(&cube.get_model_transform()),
                view:        na4_to_gl4(&view),
                projection:  na4_to_gl4(&projection.as_matrix()),
                colors:      &color_buffer,
                lightColor:  light_color,
                objectColor: object_color,
                lightPos:    light_position,
            };
            let program = match cube.get_type() {
                &CubeType::Block => &block_program,
                &CubeType::Light => &light_program
            };
            target.draw(cube.get_vert_buffer(), &indices, program, &uniforms, &params).unwrap();
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
                     glutin::WindowEvent::KeyboardInput{device_id, input} => match input.scancode {
                         19 => { rotate_cubes = !rotate_cubes; },
                         _ => println!("{:?}", input)
                     }
                    _ => ()
                },
                _ => (),
            }
        });
        d += 0.001;
    }

}

// fn get_x_rot(d: f32) -> Matrix4<f32> {
//     let cd = f32::cos(d);
//     let sd = f32::sin(d);
//     Matrix4::new(
//         1.0, 0.0, 0.0, 0.0,
//         0.0,  cd, -sd, 0.0,
//         0.0,  sd,  cd, 0.0,
//         0.0, 0.0, 0.0, 1.0
//     )
// }

// fn get_y_rot(d: f32) -> Matrix4<f32> {
//     let cd = f32::cos(d);
//     let sd = f32::sin(d);
//     Matrix4::new(
//          cd, 0.0,  sd, 0.0,
//         0.0, 1.0, 0.0, 0.0,
//         -sd, 0.0,  cd, 0.0,
//         0.0, 0.0, 0.0, 1.0
//     )
// }

// fn get_z_rot(d: f32) -> Matrix4<f32> {
//     let cd = f32::cos(d);
//     let sd = f32::sin(d);
//     Matrix4::new(
//          cd, -sd, 0.0, 0.0,
//          sd,  cd, 0.0, 0.0,
//         0.0, 0.0, 1.0, 0.0,
//         0.0, 0.0, 0.0, 1.0
//     )
// }

// fn get_identity_matrix() -> Matrix4<f32> {
//     Matrix4::new(
//         1.0, 0.0, 0.0, 0.0,
//         0.0, 1.0, 0.0, 0.0,
//         0.0, 0.0, 1.0, 0.0,
//         0.0, 0.0, 0.0, 1.0
//     )
// }
 
fn na4_to_gl4(mat: &Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[0],  mat[1],  mat[2],  mat[3]],
        [mat[4],  mat[5],  mat[6],  mat[7]],
        [mat[8],  mat[9],  mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
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