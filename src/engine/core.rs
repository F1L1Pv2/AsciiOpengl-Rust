use super::ascii_render::{ Color, TerminalFrameBuffer };
use super::camera::Camera;
// use super::matrices::{ model_matrix };
use super::prefab::{get_prefabs, PrefabList};
use terminal_size::terminal_size;
use glium::{ glutin};

//create init type

type InitType = (
    (u32, u32),
    TerminalFrameBuffer,
    glutin::event_loop::EventLoop<()>,
    glium::Display,
    glium::Program,
    glium::DrawParameters<'static>,
    Camera,
    PrefabList
);


pub fn init() -> InitType
{

    let terminal_res = terminal_size().unwrap();
    let terminal_res: (u32, u32) = (u32::from(terminal_res.0.0), u32::from(terminal_res.1.0));

    let terminal_fb = TerminalFrameBuffer::new(
        (terminal_res.0 as usize) / 2,
        terminal_res.1 as usize,
        Color { r: 0, g: 0, b: 0 }
    );

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder
        ::new()
        .with_visible(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(terminal_res.0, terminal_res.1));

    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //read vertex shader source code from file
    let vertex_shader_src = std::fs
        ::read_to_string("src/shaders/vertex_shader.glsl")
        .expect("Failed to read vertex shader source code from file");

    let fragment_shader_src = std::fs
        ::read_to_string("src/shaders/fragment_shader.glsl")
        .expect("Failed to read fragment shader source code from file");

    let program = glium::Program
        ::from_source(&display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None)
        .unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    


    let camera = Camera::new([0.0, 0.0, 0.0f32], [0.0, 0.0, 0.0f32], 0.05, 0.05, terminal_res);

    let prefab_list = get_prefabs();

    (terminal_res, terminal_fb, event_loop, display, program, params, camera,prefab_list)

}