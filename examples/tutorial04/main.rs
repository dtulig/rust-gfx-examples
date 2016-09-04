extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, vec3, Deg, Rad};
use gfx::traits::FactoryExt;
use gfx::Device;
use time::precise_time_s;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "coord3d",
        color: [f32; 3] = "v_color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        m_transform: gfx::Global<[[f32; 4]; 4]> = "m_transform",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.8, -0.8, 0.0 ], color: [ 1.0, 0.0, 0.0 ] },
    Vertex { pos: [  0.8, -0.8, 0.0 ], color: [ 0.0, 1.0, 0.0 ] },
    Vertex { pos: [  0.0,  0.8, 0.0 ], color: [ 0.0, 0.0, 1.0 ] }
];

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("My First Triangle".to_string())
        .with_dimensions(640, 480);

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("triangle_120.glslv"),
        include_bytes!("triangle_120.glslf"),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    let mut data = pipe::Data {
        vbuf: vertex_buffer.clone(),
        out: main_color.clone(),
        m_transform: Matrix4::from_scale(1.0).into(),
    };

    'main: loop {
        for ev in window.poll_events() {
            match ev {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }

        let tmove = (precise_time_s() * (2.0*3.14) / 5.0).sin(); // -1 <-> +1 every 5 seconds
        let angle = precise_time_s() * 45.0; // 45 degrees per second

        let m_rotate: Matrix4<f32> = Matrix4::from_angle_z(Rad::from(Deg(angle as f32)));
        let m_transform: Matrix4<f32> = Matrix4::from_translation(vec3(tmove as f32, 0.0, 0.0));

        data.m_transform = (m_transform * m_rotate).into();

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
