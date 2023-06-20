use super::ascii_render::{ Color, TerminalFrameBuffer };
use super::camera::Camera;
use super::object::Object;
// use super::matrices::{ model_matrix };
// use super::prefab::{get_prefabs, PrefabList};
use terminal_size::terminal_size;
use glium::{ glutin};
use super::scene::Scene;

//create init type

type InitType = (
    (u32, u32),
    TerminalFrameBuffer,
    glutin::event_loop::EventLoop<()>,
    glium::Display,
    glium::Program,
    glium::Program,
    glium::DrawParameters<'static>,
    glium::DrawParameters<'static>,
    Camera
);

#[derive(Debug)]
pub struct UiElems {
    pub elems: Vec<Object>,
}

pub struct Game {
    scenes: Vec<Scene>,
    ui_elems: UiElems,
    current_scene: usize,
}

#[allow(dead_code)]
impl Game {
    pub fn new() -> Game {
        Game {
            scenes: Vec::new(),
            ui_elems: UiElems { elems: Vec::new() },
            current_scene: 0,
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn set_scene(&mut self, scene: usize) {
        self.current_scene = scene;
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scenes[self.current_scene]
    }

    pub fn get_scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene]
    }

    pub fn add_ui_elem(&mut self, elem: Object) {
        self.ui_elems.elems.push(elem);
    }

    pub fn add_ui_elems(&mut self, elems: Vec<Object>) {
        for elem in elems {
            self.ui_elems.elems.push(elem);
        }
    }

    pub fn get_ui_elems(&self) -> &UiElems {
        &self.ui_elems
    }
}


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
        ::read_to_string("assets/shaders/vertex_shader.glsl")
        .expect("Failed to read vertex shader source code from file");

    let fragment_shader_src = std::fs
        ::read_to_string("assets/shaders/fragment_shader.glsl")
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
        blend: glium::Blend::alpha_blending(),
        //set texture filtering to nearest

        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let ui_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        //set texture filtering to nearest

        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let ui_vertex_shader_src = std::fs
        ::read_to_string("assets/shaders/ui_vertex.glsl")
        .expect("Failed to read vertex shader source code from file");

    let ui_fragment_shader_src = std::fs
        ::read_to_string("assets/shaders/ui_fragment.glsl")
        .expect("Failed to read fragment shader source code from file");


    let ui_program = glium::Program
        ::from_source(&display, ui_vertex_shader_src.as_str(), ui_fragment_shader_src.as_str(), None)
        .unwrap();

    


    let camera = Camera::new([0.0, 0.0, 0.0f32], [0.0, 0.0, 0.0f32], 0.05, 0.05, terminal_res);

    (terminal_res, terminal_fb, event_loop, display, program,ui_program, params,ui_params, camera)

}