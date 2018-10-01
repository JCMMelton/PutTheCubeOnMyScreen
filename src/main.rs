#[macro_use]
extern crate glium;
extern crate shader_version;
extern crate glutin_window;
extern crate gl;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate ncollide3d;
extern crate nphysics3d;


mod vertex;
mod cube;
mod input;
mod cubody;
mod world_force;

use glium::*;
use na::{Matrix4, geometry, Vector3, Vector6, Vector2, Isometry3, Point3};
use glm::*;
use std::f32;
use ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle, Material, RigidBody};
use nphysics3d::volumetric::Volumetric;
use nphysics3d::world::World;
use nphysics3d::force_generator::*;
use nphysics3d::algebra::Force3;

use cube::*;
use input::*;
use cubody::*;
use world_force::*;

const COLLIDER_MARGIN: f32 = 0.01;

fn main() {
    use glium::{glutin, Surface};
    let mut dimensions: [f32; 2] = [800.0, 600.0];
    let mut event_loop = glutin::EventsLoop::new();

    // let monitor = event_loop.get_available_monitors().nth(1);

    let mut window = glutin::WindowBuilder::new();//.with_fullscreen(monitor);
    window.window.dimensions = Some(glutin::dpi::LogicalSize::new(dimensions[0] as f64, dimensions[1] as f64));
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display: Display = glium::Display::new(window, context, &event_loop).unwrap();
    display.gl_window().hide_cursor(true);
    //display.gl_window().grab_cursor(true);

    let mut world = World::<f32>::new();
    world.set_gravity(Vector3::new(0.0, 0.0, 0.0));
    let mut world_force = WorldForce::new(Vec::new());
    let mut implode = Implosion::new(Vec::new(), Point3::new(0.0, 0.0, 0.0), 6.0);
    let mut planet  = Implosion::new(Vec::new(), Point3::new(20.0, 20.0, 20.0), 15.0);
    let mut planet2  = Implosion::new(Vec::new(), Point3::new(-20.0, -20.0, -20.0), 15.0);
    let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(0.5-COLLIDER_MARGIN)));
    let inertia = geom.inertia(1.1);
    let center_of_mass = geom.center_of_mass();

    let light_position: (f32, f32, f32) = (0.0, 0.0, 0.0);
    let cube_iter = 15;
    let mut cubes: Vec<Cubody> = Vec::new();
    let cube_resolution = 1.0;
    for i in -cube_iter..cube_iter {
        for j in -cube_iter..cube_iter {
            for k in 0..1 {
                let fi = i as f32;
                let fj = j as f32;
                let fk = k as f32;
                let hpi = 1.0/(0.1+hypot(fi, fj));
                let hpj = 1.0/(0.1+hypot(fk, fj));
                let hpk = 1.0/(0.1+hypot(fi, fk));
                let location: Vector3<f32> = Vector3::new(fi*cube_resolution, fj*cube_resolution, 0.0 + (fk*cube_resolution));
                let pos = Isometry3::new(location, na::zero());
                let handle = world.add_rigid_body(pos, inertia, center_of_mass);
                cubes.push(
                    Cubody::new(
                        Cube::new(
                            CubeType::Block,
                            pos,
                            [hpi, hpj, hpk],
                            cube_resolution*0.5,
                            &display
                        ),
                        handle
                    )
                );
                world.add_collider(
                    COLLIDER_MARGIN,
                    geom.clone(),
                    handle,
                    Isometry3::identity(),
                    Material::default()
                );
                world_force.add_body_part(handle);
                implode.add_body_part(handle);
                planet.add_body_part(handle);
                planet2.add_body_part(handle);
            }
        }
    }

    world.add_force_generator(world_force);
    world.add_force_generator(implode);
    world.add_force_generator(planet);
    world.add_force_generator(planet2);
//    cubes.push(Cube::new(
//        CubeType::Light,
//        Vector3::new(light_position.0, light_position.1, light_position.2),
//        [1.0, 1.0, 1.0],
//        1.0,
//        &display
//    ));

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let block_vertex_shader_src   = include_str!("../assets/block.vert");
    let block_fragment_shader_src = include_str!("../assets/block.frag");
    let block_program: Program = glium::Program::from_source(&display, block_vertex_shader_src, block_fragment_shader_src, None).unwrap();

    let light_vertex_shader_src   = include_str!("../assets/light.vert");
    let light_fragment_shader_src = include_str!("../assets/light.frag");
    let light_program: Program = glium::Program::from_source(&display, light_vertex_shader_src, light_fragment_shader_src, None).unwrap();

    let projection = geometry::Perspective3::new(dimensions[0]/dimensions[1], f32::consts::PI/2.0, 0.1, 1000.0);

    let expl = 0.1;
    let mut closed = false;
    let mut d: f32 = 0.001;

    let light_color:  [f32; 3] = [1.0, 1.0, 1.0];

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
    let mut rotate_cubes: bool = false;
    let mut mouse_offset: Vector2<f32> = Vector2::new(0.0, 0.0);
    let mouse_sensitivity: f32 = 0.5;
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = -88.0;

    let camera_speed: f32 = 1.0;
    let mut camera_pos = glm::vec3(0.0, 0.0, 20.0);
    let camera_up  = glm::vec3(0.0, 1.0, 0.0);

    let mut input_holder: Input = Input::new();

    while !closed {

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        yaw   += mouse_offset.x;
        pitch += mouse_offset.y;
        if pitch > 89.0 {
            pitch = 89.0;
        }
        if pitch < -89.0 {
            pitch = -89.0;
        }

        let front = glm::vec3(
            f32::cos(radianize(&yaw)) * f32::cos(radianize(&pitch)),
            f32::sin(radianize(&pitch)),
            f32::sin(radianize(&yaw)) * f32::cos(radianize(&pitch))
        );
        let camera_front = glm::normalize(&front);

        let view = glm::look_at(
            &camera_pos,
            &(camera_pos+camera_front),
            &camera_up
        );

        for cubody in cubes.iter_mut() {
            let cubody_pos = world.rigid_body(cubody.handle).unwrap().position();
            let mut rigid_body: &mut RigidBody<f32> = world.rigid_body_mut(cubody.handle).unwrap();
            cubody.cube.move_location(cubody_pos);
            
            let uniforms = uniform!{
                window_size: dimensions,
                model:       na4_to_gl4(&cubody.cube.get_model_transform()),
                view:        na4_to_gl4(&view),
                projection:  na4_to_gl4(&projection.as_matrix()),
                lightColor:  light_color,
                objectColor: cubody.cube.get_color(),
                lightPos:    light_position,
            };
            let program = match cubody.cube.get_type() {
                &CubeType::Block => &block_program,
                &CubeType::Light => &light_program
            };
            target.draw(cubody.cube.get_vert_buffer(), &indices, program, &uniforms, &params).unwrap();
        }
        target.finish().unwrap();

        mouse_offset.x = 0.0;
        mouse_offset.y = 0.0;

        event_loop.poll_events(|event| {
            match event {
                glutin::Event::DeviceEvent { event, ..} => match event {
                    glutin::DeviceEvent::MouseMotion{ delta }  => match delta {
                        _ => {
                            mouse_offset.x = ( delta.0 as f32) * mouse_sensitivity;
                            mouse_offset.y = (-delta.1 as f32) * mouse_sensitivity;
                        }
                    },
                    _ => ()
                }
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::Resized(size) => {
                        dimensions[0] = size.width  as f32;
                        dimensions[1] = size.height as f32;
                    },
                    glutin::WindowEvent::KeyboardInput{ input, .. } => {
                        let latch = match input.state {
                            glutin::ElementState::Pressed  => true,
                            glutin::ElementState::Released => false
                        };
                        if input.virtual_keycode == Some(glutin::VirtualKeyCode::W) {
                            input_holder.forward = latch;
                        }
                        else if input.virtual_keycode == Some(glutin::VirtualKeyCode::S) {
                            input_holder.backward = latch;
                        }
                        if input.virtual_keycode == Some(glutin::VirtualKeyCode::D) {
                            input_holder.right = latch;
                        }
                        else if input.virtual_keycode == Some(glutin::VirtualKeyCode::A) {
                            input_holder.left = latch;
                        }
                        if input.virtual_keycode == Some(glutin::VirtualKeyCode::Space) {
                            input_holder.up = latch;
                        }
                        if input.virtual_keycode == Some(glutin::VirtualKeyCode::R) {
                            input_holder.rotate = latch;
                            rotate_cubes = true;
                        }
                        if input.virtual_keycode == Some(glutin::VirtualKeyCode::Escape) {
                            closed = true;
                        }
//                        println!("{:?}", input)
                    }
                    _ => ()
                },
                _ => (),
            }
        });
        if input_holder.forward {
            camera_pos += camera_speed * camera_front;
        }
        if input_holder.backward {
            camera_pos -= camera_speed * camera_front;
        }
        if input_holder.right {
            camera_pos += glm::normalize(&glm::cross::<f32, U3>(&camera_front, &camera_up)) * camera_speed;
        }
        if input_holder.left {
            camera_pos -= glm::normalize(&glm::cross::<f32, U3>(&camera_front, &camera_up)) * camera_speed;
        }
        world.step();
        d += 0.01;
    }

}

fn radianize(n: &f32) -> f32 {
    n * (f32::consts::PI/180.0)
}

fn hypot(a: f32, b: f32) -> f32 {
    f32::sqrt(f32::powi(a, 2) + f32::powi(b, 2))
}
 
fn na4_to_gl4(mat: &Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[0],  mat[1],  mat[2],  mat[3]],
        [mat[4],  mat[5],  mat[6],  mat[7]],
        [mat[8],  mat[9],  mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ]
}
