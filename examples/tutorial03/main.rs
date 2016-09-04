#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

use gfx::traits::FactoryExt;
use gfx::Device;
use time::precise_time_s;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "coord2d",
        color: [f32; 3] = "v_color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        fade: gfx::Global<f32> = "fade",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.8, -0.8 ], color: [ 1.0, 0.0, 0.0 ] },
    Vertex { pos: [  0.8, -0.8 ], color: [ 0.0, 1.0, 0.0 ] },
    Vertex { pos: [  0.0,  0.8 ], color: [ 0.0, 0.0, 1.0 ] }
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
        fade: 0.0,
    };

    'main: loop {
        for ev in window.poll_events() {
            match ev {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }

        let fade_pct = (precise_time_s() * (2.0*3.14) / 5.0).sin() / 2.0 + 0.5;

        data.fade = fade_pct as f32;

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
