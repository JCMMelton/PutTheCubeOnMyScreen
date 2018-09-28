#[macro_use]
extern crate glium;
extern crate shader_version;
extern crate glutin_window;
extern crate gl;
extern crate nalgebra as na;
extern crate camera_controllers;


mod vertex;
mod cube;

use glium::*;
use na::{Matrix4, geometry, Vector3, Vector2, Isometry3, Point3, Perspective3};
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

    let light_position: (f32, f32, f32) = (0.0, 10.0, 5.0);
    let cube_iter = 10;
    let mut cubes: Vec<Cube> = Vec::new();
    for i in -cube_iter..cube_iter {
        for j in -cube_iter..cube_iter {
            let fi = i as f32;
            let fj = j as f32;
            cubes.push(Cube::new(
                CubeType::Block,
                Vector3::new(fi*1.0, fj*1.0, 10.0),
                [f32::abs(1.0/fi), f32::abs(1.0/fj), 0.3],
                0.5,
                &display
            ));
        }
    }
    // cubes.push(Cube::new(
    //     CubeType::Block,
    //     Vector3::new(2.0, -1.0, 10.0),
    //     &display
    // ));
    // cubes.push(Cube::new(
    //     CubeType::Block,
    //     Vector3::new(0.0, 0.0, 10.0),
    //     &display
    // ));
    // cubes.push(Cube::new(
    //     CubeType::Block,
    //     Vector3::new(-2.0, 1.0, 10.0),
    //     &display
    // ));
    cubes.push(Cube::new(
        CubeType::Light,
        Vector3::new(light_position.0, light_position.1, light_position.2),
        [1.0, 1.0, 1.0],
        1.0,
        &display
    ));

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let block_vertex_shader_src   = include_str!("../assets/block.vert");
    let block_fragment_shader_src = include_str!("../assets/block.frag");
    let block_program: Program = glium::Program::from_source(&display, block_vertex_shader_src, block_fragment_shader_src, None).unwrap();

    let light_vertex_shader_src   = include_str!("../assets/light.vert");
    let light_fragment_shader_src = include_str!("../assets/light.frag");
    let light_program: Program = glium::Program::from_source(&display, light_vertex_shader_src, light_fragment_shader_src, None).unwrap();

    let projection = geometry::Perspective3::new(dimensions[0]/dimensions[1], f32::consts::PI/2.0, 0.1, 1000.0);


    // let player_pos = [0.0, 0.0, 0.0];
    // let player_yaw = 0.0;
    // let player_pitch = 0.0;
    // let mut first_person_settings = camera_controllers::FirstPersonSettings::keyboard_wasd();
    // first_person_settings.mouse_sensitivity_horizontal = 0.5;
    // first_person_settings.mouse_sensitivity_vertical = 0.5;
    // first_person_settings.speed_horizontal = 8.0;
    // first_person_settings.speed_vertical = 4.0;
    // let mut first_person = camera_controllers::FirstPerson::new(
    //     player_pos,
    //     first_person_settings
    // );
    // first_person.yaw = f32::consts::PI - player_yaw / 180.0 * f32::consts::PI;
    // first_person.pitch = player_pitch / 180.0 * f32::consts::PI;


    let mut closed = false;
    let mut d: f32 = 0.001;

    let mut targ = Point3::new(0.0, 0.0, 1.0);
    let mut eye  = Point3::new(0.0, 0.0, 0.0);

    let light_color:  [f32; 3] = [1.0, 1.0, 1.0];
    let object_color: [f32; 3] = [1.0, 0.5, 0.3];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
    let mut rotate_cubes: bool = false;
    let mut mouse_position: Vector2<f32> = Vector2::new(0.0, 0.0);

    while !closed {

        let mut target = display.draw();
        target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

        let view:  Matrix4<f32> = Isometry3::look_at_rh(&eye, &targ, &Vector3::y()).to_homogeneous();

        for cube in cubes.iter_mut() {

            if rotate_cubes {
                match cube.get_type() {
                    CubeType::Block => {
                        let f = 1.0 + f32::sqrt( f32::powf(cube.get_x_pos(), 2.0) + f32::powf(cube.get_y_pos(), 2.0));
                        cube.rotate(f32::cos(d), f32::sin(d), d/f);
                        cube.move_location(Vector3::new(0.0, 0.0, f32::sin(d+f)/100.0));
                    },
                    _ => ()
                };
            }
            
            let uniforms = uniform!{
                window_size: dimensions,
                model:       na4_to_gl4(&cube.get_model_transform()),
                view:        na4_to_gl4(&view),
                projection:  na4_to_gl4(&projection.as_matrix()),
                lightColor:  light_color,
                objectColor: cube.get_color(),
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
                    glutin::WindowEvent::KeyboardInput{device_id, input} => match input.virtual_keycode {
                        Some(glutin::VirtualKeyCode::W) => {
                            eye.z  += 0.1;
                            targ.z += 0.1;
                        }
                        Some(glutin::VirtualKeyCode::S) => {
                            eye.z  -= 0.1;
                            targ.z -= 0.1;
                        },
                        Some(glutin::VirtualKeyCode::D) => {
                            eye.x  -= 0.1;
                            targ.x -= 0.1;
                        },
                        Some(glutin::VirtualKeyCode::A) => {
                            eye.x  += 0.1;
                            targ.x += 0.1;
                        },
                        Some(glutin::VirtualKeyCode::R) => {
                            rotate_cubes = !rotate_cubes;
                        },
                        _ => println!("{:?}", input)
                    }
                    glutin::WindowEvent::CursorMoved{device_id, position, modifiers} => match position {
                        _ => {
                            mouse_position.x = position.x as f32;
                            mouse_position.y = position.y as f32;
                        }
                    }
                    _ => ()
                },
                _ => (),
            }
        // first_person.event(&event);
        });
        d += 0.01;
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
