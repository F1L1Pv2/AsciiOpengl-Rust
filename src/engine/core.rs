// use std::fmt::Display;

use super::ascii_render::{Color, TerminalFrameBuffer};
use super::camera::Camera;
use super::object::{Object, TextureFilter};
// use super::matrices::{ model_matrix };
// use super::prefab::{get_prefabs, PrefabList};
use device_query::DeviceState;
use glium::glutin;
use glium::Surface;
use terminal_size::terminal_size;
// use super::game_loop::game_loop;
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
    Game,
);

#[derive(Debug)]
pub struct UiElems {
    pub elems: Vec<Object>,
}

pub struct Game {
    scenes: Vec<Scene>,
    pub camera: Camera,
    ui_elems: UiElems,
    current_scene: usize,
    pub delta_time: f32,
    pub assets_path: String,
}

#[allow(dead_code)]
impl Game {
    pub fn new(camera: Camera, assets_path: String) -> Game {
        Game {
            scenes: Vec::new(),
            ui_elems: UiElems { elems: Vec::new() },
            camera,
            current_scene: 0,
            delta_time: 0.0,
            assets_path,
        }
    }

    /// Add a scene to the game
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    /// Set the current scene
    pub fn set_scene(&mut self, scene: usize) {
        self.current_scene = scene;
    }

    /// Get the current scene index
    pub fn get_scene_index(&self) -> usize {
        self.current_scene
    }

    /// Get the current scene
    pub fn get_scene(&self) -> &Scene {
        &self.scenes[self.current_scene]
    }

    /// Get the current scene as mutable
    pub fn get_scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene]
    }

    /// Get scene by index
    pub fn get_scene_by_index(&self, index: usize) -> &Scene {
        &self.scenes[index]
    }

    /// Get scene by index as mutable
    pub fn get_scene_by_index_mut(&mut self, index: usize) -> &mut Scene {
        &mut self.scenes[index]
    }

    /// add a ui element to the game
    pub fn add_ui_elem(&mut self, elem: Object) {
        self.ui_elems.elems.push(elem);
    }

    /// add a vector of ui elements to the game
    pub fn add_ui_elems(&mut self, elems: Vec<Object>) {
        for elem in elems {
            self.ui_elems.elems.push(elem);
        }
    }

    /// get the ui elements
    pub fn get_ui_elems(&self) -> &UiElems {
        &self.ui_elems
    }

    /// get the ui elements as mutable
    pub fn get_ui_elems_mut(&mut self) -> &mut UiElems {
        &mut self.ui_elems
    }
}

pub fn init(assets_path: &str) -> InitType {
    let terminal_res = terminal_size().unwrap();
    let terminal_res: (u32, u32) = (u32::from(terminal_res.0 .0), u32::from(terminal_res.1 .0));

    let terminal_fb = TerminalFrameBuffer::new(
        (terminal_res.0 as usize) / 2,
        terminal_res.1 as usize,
        Color { r: 0, g: 0, b: 0 },
    );

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_visible(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            terminal_res.0,
            terminal_res.1,
        ));

    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //read vertex shader source code from file
    // let vertex_shader_src = std::fs::read_to_string("assets/shaders/vertex_shader.glsl")
        // .expect("Failed to read vertex shader source code from file");

    let vertex_path = format!("{}/shaders/vertex_shader.glsl", assets_path);
    let vertex_shader_src = std::fs::read_to_string(vertex_path)
        .expect("Failed to read vertex shader source code from file");

    // let fragment_shader_src = std::fs::read_to_string("assets/shaders/fragment_shader.glsl")
        // .expect("Failed to read fragment shader source code from file");

    let fragment_path = format!("{}/shaders/fragment_shader.glsl", assets_path);
    let fragment_shader_src = std::fs::read_to_string(fragment_path)
        .expect("Failed to read fragment shader source code from file");

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    )
    .unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        //set texture filtering to nearest
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let ui_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        //set texture filtering to nearest
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    // let ui_vertex_shader_src = std::fs::read_to_string("assets/shaders/ui_vertex.glsl")
    //     .expect("Failed to read vertex shader source code from file");

    let ui_vertex_path = format!("{}/shaders/ui_vertex.glsl", assets_path);
    let ui_vertex_shader_src = std::fs::read_to_string(ui_vertex_path)
        .expect("Failed to read vertex shader source code from file");

    // let ui_fragment_shader_src = std::fs::read_to_string("assets/shaders/ui_fragment.glsl")
    //     .expect("Failed to read fragment shader source code from file");

    let ui_fragment_path = format!("{}/shaders/ui_fragment.glsl", assets_path);
    let ui_fragment_shader_src = std::fs::read_to_string(ui_fragment_path)
        .expect("Failed to read fragment shader source code from file");

    let ui_program = glium::Program::from_source(
        &display,
        ui_vertex_shader_src.as_str(),
        ui_fragment_shader_src.as_str(),
        None,
    )
    .unwrap();

    let camera = Camera::new(
        [0.0, 0.0, 0.0f32],
        [0.0, 0.0, 0.0f32],
        0.05,
        0.05,
        terminal_res,
    );

    let game = Game::new(camera, assets_path.to_string());

    (
        terminal_res,
        terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        game,
    )
}

#[macro_export]
/// The `game_loop` macro creates a game loop function with ability to pass additional parameters.
/// NOTE: The game loop function must have the following signature:
/// `fn(&DeviceState, (u32, u32), &mut Game, &glium::Display, ...)`
/// where `...` is the list of additional parameters.
macro_rules! game_loop {
    ($game_loop_func:expr $(, $param:expr)* $(,)?) => {
        move |device_state, terminal_res, game, display| {
            $game_loop_func(device_state, terminal_res, game, display $(, $param)*);
        }
    };
}

#[macro_export]
/// The `game_init` macro creates a game init function with ability to pass additional parameters.
/// NOTE: The game init function must have the following signature:
/// `fn( (u32,u32), &mut Game, &glium::Display, ...)`
/// where `...` is the list of additional parameters.
macro_rules! game_init {
    ($game_init_func:expr $(, $param:expr)* $(,)?) => {
        move |terminal_res,game, display| {
            $game_init_func(terminal_res,game, display $(, $param)*);
        }
    };
}

#[macro_export]
/// The `init_engine` macro creates a game loop function and a game init function and runs the game.
/// NOTE: You need to pass the path to the assets folder as the third argument.
/// NOTE: recommended to use macro `game_loop!` to create the game loop function and `game_init!` to create the game init function.
macro_rules! init_engine {
    ($game_loop_func:expr, $game_init_func:expr, $assets_path:expr) => {
        ascii_opengl_rust::engine::core::run_event_loop(ascii_opengl_rust::engine::core::init($assets_path), $game_loop_func, $game_init_func);
    };
}

pub fn run_event_loop<F, G>(init_type: InitType, mut game_loop: F, mut game_init: G)
where
    F: FnMut(&DeviceState, (u32, u32), &mut Game, &glium::Display) + 'static,
    G: FnMut((u32, u32),&mut Game, &glium::Display) + 'static,
{
    let (
        terminal_res,
        terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        mut game,
    ) = init_type;

    game_init(terminal_res,&mut game, &display);

    let mut terminal_res = terminal_res;
    let mut terminal_fb = terminal_fb;

    let light = [1.4, 0.4, -0.7f32];
    let device_state = DeviceState::new();
    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let texture = glium::texture::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            terminal_res.0,
            terminal_res.1,
        )
        .unwrap();

        // Create a depth buffer for off-screen rendering
        let depthbuffer = glium::framebuffer::DepthRenderBuffer::new(
            &display,
            glium::texture::DepthFormat::F32,
            terminal_res.0,
            terminal_res.1,
        )
        .unwrap();

        // Create a framebuffer for off-screen rendering
        let mut framebuffer: glium::framebuffer::SimpleFrameBuffer =
            glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &display,
                &texture,
                &depthbuffer,
            )
            .unwrap();

        // Check res and update if changed
        let new_terminal_res = terminal_size().unwrap();
        let new_terminal_res: (u32, u32) = (
            u32::from(new_terminal_res.0 .0),
            u32::from(new_terminal_res.1 .0),
        );
        if new_terminal_res != terminal_res {
            terminal_res = new_terminal_res;
            terminal_fb = TerminalFrameBuffer::new(
                (terminal_res.0 as usize) / 2,
                terminal_res.1 as usize,
                Color { r: 0, g: 0, b: 0 },
            );
            TerminalFrameBuffer::update_res(
                &mut terminal_fb,
                new_terminal_res.0 as usize,
                new_terminal_res.1 as usize,
            );
        }

        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }

            glutin::event::Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                accumulator += now - next_frame_time;
                next_frame_time = now;

                game.delta_time = accumulator.as_secs_f32();

                while accumulator >= fixed_timestep {
                    //--------------------------------- Sort of a game loop ---------------------------------

                    game_loop(&device_state, terminal_res, &mut game, &display);

                    accumulator -= fixed_timestep;
                }

                //--------------------------------- Render (post update) ---------------------------------

                framebuffer
                    .clear_color_and_depth((105.0 / 255.0, 109.0 / 255.0, 219.0 / 255.0, 1.0), 1.0);

                for object in &game.get_scene().objects {
                    let uniforms = uniform! {
                        model: object.model,
                        view: game.camera.view_matrix(),
                        perspective: game.camera.perspective_matrix(),
                        u_light: light,
                        tex: &object.texture,
                    };

                    framebuffer
                        .draw(&object.vb, &object.ib, &program, &uniforms, &params)
                        .unwrap();
                }

                //--------------------------------- UI ---------------------------------

                for ui_elem in &game.get_ui_elems().elems {
                    let behavior = match ui_elem.texture_filter {
                        TextureFilter::Nearest => glium::uniforms::SamplerBehavior {
                            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
                            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                            ..Default::default()
                        },
                        TextureFilter::Linear => glium::uniforms::SamplerBehavior {
                            minify_filter: glium::uniforms::MinifySamplerFilter::Linear,
                            magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
                            ..Default::default()
                        },
                    };

                    let uniforms = uniform! {
                        tex: glium::uniforms::Sampler(&ui_elem.texture, behavior),
                        // tex: &ui_elem.texture,
                    };

                    framebuffer
                        .draw(&ui_elem.vb, &ui_elem.ib, &ui_program, &uniforms, &ui_params)
                        .unwrap();
                }
            }

            _ => {
                return;
            }
        }

        //get pixels from display
        //TODO: use framebuffer.read_to_pixel_buffer
        let pixels: glium::texture::RawImage2d<u8> = texture.read();
        terminal_fb.clear();
        for i in 0..pixels.data.len() / 4 {
            let r = pixels.data[i * 4];
            let g = pixels.data[i * 4 + 1];
            let b = pixels.data[i * 4 + 2];

            let mut x = ((i % (pixels.width as usize)) * (terminal_res.0 as usize))
                / (pixels.width as usize);
            if x >= (terminal_res.0 as usize) / 4 {
                x -= (terminal_res.0 as usize) / 4;
            } else {
                x = 0;
            }
            let y = (terminal_res.1 as usize)
                - ((i / (pixels.width as usize)) * (terminal_res.1 as usize))
                    / (pixels.height as usize);

            let color = Color { r, g, b };

            terminal_fb.set_pixel(x, y, color);
        }
        terminal_fb.draw_frame();
    });
}
