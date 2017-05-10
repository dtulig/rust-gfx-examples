extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

use cgmath::{Matrix4, vec3, Point3, Deg, Rad, perspective};
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
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
        mvp: gfx::Global<[[f32; 4]; 4]> = "mvp",
    }
}

const CUBE: [Vertex; 8] = [
    // front
    Vertex { pos: [ -1.0, -1.0, 1.0 ], color: [ 1.0, 0.0, 0.0 ] },
    Vertex { pos: [  1.0, -1.0, 1.0 ], color: [ 0.0, 1.0, 0.0 ] },
    Vertex { pos: [  1.0,  1.0, 1.0 ], color: [ 0.0, 0.0, 1.0 ] },
    Vertex { pos: [ -1.0,  1.0, 1.0 ], color: [ 1.0, 1.0, 1.0 ] },
    // back
    Vertex { pos: [ -1.0, -1.0, -1.0 ], color: [ 1.0, 0.0, 0.0 ] },
    Vertex { pos: [  1.0, -1.0, -1.0 ], color: [ 0.0, 1.0, 0.0 ] },
    Vertex { pos: [  1.0,  1.0, -1.0 ], color: [ 0.0, 0.0, 1.0 ] },
    Vertex { pos: [ -1.0,  1.0, -1.0 ], color: [ 1.0, 1.0, 1.0 ] }
];


fn main() {
    let mut width = 640;
    let mut height = 480;

    let builder = glutin::WindowBuilder::new()
        .with_title("My First Triangle".to_string())
        .with_dimensions(width, height);

    let events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    // Despite requesting 640x480, verify the height and width.
    let result = window.get_inner_size_pixels().unwrap();
    width = result.0;
    height = result.1;

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("triangle_120.glslv"),
        include_bytes!("triangle_120.glslf"),
        pipe::new()
    ).unwrap();

    let cube_elements: &[u16] = &[
        // front
        0, 1, 2,
        2, 3, 0,
        // top
        1, 5, 6,
        6, 2, 1,
        // back
        7, 6, 5,
        5, 4, 7,
        // bottom
        4, 0, 3,
        3, 7, 4,
        // left
        4, 5, 1,
        1, 0, 4,
        // right
        3, 2, 6,
        6, 7, 3
    ];

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&CUBE, cube_elements);

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
        out_depth: main_depth,
        mvp: Matrix4::from_scale(1.0).into(),
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|glutin::Event::WindowEvent{window_id: _, event}| {
            match event {
                glutin::WindowEvent::Resized(w, h) => {
                    width = w;
                    height = h;
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut data.out_depth);
                }
                glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) |
                glutin::WindowEvent::Closed => running = false,
                _ => ()
            }
        });

        let model = Matrix4::from_translation(vec3(0.0, 0.0, -4.0));
        let view = Matrix4::look_at(Point3::new(0.0, 2.0, 0.0), Point3::new(0.0, 0.0, -4.0), vec3(0.0, 1.0, 0.0));
        let projection = perspective(Deg(45.0), 1.0 * (width as f32)/(height as f32), 0.1, 10.0);

        let angle = precise_time_s() * 45.0; // 45 degrees per second
        let anim = Matrix4::from_angle_y(Rad::from(Deg(angle as f32)));

        let mvp = projection * view * model * anim;

        data.mvp = mvp.into();

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
