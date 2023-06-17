#[macro_use]
extern crate glium;

//--------------Terminal Stuff -------------------------
use device_query::{ DeviceQuery, DeviceState, Keycode };
use terminal_size::{ terminal_size };
//------------------ Teapot ----------------------------
mod rawmodels;
use rawmodels::teapot;
//------------------ My stuff --------------------------
mod engine;
use engine::camera::Camera;
use engine::ascii_render::{ TerminalFrameBuffer, Color };
//------------------------------------------------------

fn main() {
    #[allow(unused_imports)]
    use glium::{ glutin, Surface };

    let terminal_size = terminal_size().unwrap();
    let terminal_size: (u32, u32) = (u32::from(terminal_size.0.0), u32::from(terminal_size.1.0));

    let mut terminal_fb = TerminalFrameBuffer::new(
        (terminal_size.0 as usize) / 2,
        terminal_size.1 as usize,
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    );

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder
        ::new()
        .with_visible(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(terminal_size.0, terminal_size.1));

    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer
        ::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES)
        .unwrap();

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

    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let device_state = DeviceState::new();

    let mut camera = Camera::new(
        [0.0, 0.0, 0.0f32],
        [0.0, 0.0, 0.0f32],
        0.05,
        0.05,
        terminal_size
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        let mut move_vector = [0, 0, 0];
        let mut mouse_vector = [0, 0];

        let texture = glium::texture::Texture2d
            ::empty_with_format(
                &display,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                terminal_size.0,
                terminal_size.1
            )
            .unwrap();

        // Create a depth buffer for off-screen rendering
        let depthbuffer = glium::framebuffer::DepthRenderBuffer
            ::new(&display, glium::texture::DepthFormat::F32, terminal_size.0, terminal_size.1)
            .unwrap();

        // Create a framebuffer for off-screen rendering
        let mut framebuffer = glium::framebuffer::SimpleFrameBuffer
            ::with_depth_buffer(&display, &texture, &depthbuffer)
            .unwrap();

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

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

                while accumulator >= fixed_timestep {

//--------------------------------- Sort of a game loop ---------------------------------

                    let keys: Vec<Keycode> = device_state.get_keys();

                    for key in keys {
                        match key {
                            Keycode::W => {
                                move_vector[2] = 1;
                            }
                            Keycode::S => {
                                move_vector[2] = -1;
                            }
                            Keycode::A => {
                                move_vector[0] = -1;
                            }
                            Keycode::D => {
                                move_vector[0] = 1;
                            }
                            Keycode::Space => {
                                move_vector[1] = 1;
                            }
                            Keycode::LShift => {
                                move_vector[1] = -1;
                            }
                            Keycode::I => {
                                mouse_vector[1] = 1;
                            }
                            Keycode::K => {
                                mouse_vector[1] = -1;
                            }
                            Keycode::J => {
                                mouse_vector[0] = -1;
                            }
                            Keycode::L => {
                                mouse_vector[0] = 1;
                            }
                            _ => (),
                        }
                    }
                    camera.update(terminal_size, move_vector, mouse_vector);
                    accumulator -= fixed_timestep;
                }


//--------------------------------- Render (post update) ---------------------------------

                // framebuffer.clear_color_and_depth((105./255., 109./255., 219./255., 1.0), 1.0);
                framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                let uniforms =
                    uniform! {
                    model: model,
                    view: camera.view_matrix(),
                    perspective: camera.perspective_matrix(),
                    u_light: [-1.0, 0.4, 0.9f32],
                };

                // target
                framebuffer
                    .draw((&positions, &normals), &indices, &program, &uniforms, &params)
                    .unwrap();
            }

            _ => {
                return;
            }
        }

        //get pixels from display
        let pixels: glium::texture::RawImage2d<u8> = texture.read();
        terminal_fb.clear();
        for i in 0..pixels.data.len() / 4 {
            let r = pixels.data[i * 4];
            let g = pixels.data[i * 4 + 1];
            let b = pixels.data[i * 4 + 2];

            let x =
                ((i % (pixels.width as usize)) * (terminal_size.0 as usize)) /
                    (pixels.width as usize) -
                (terminal_size.0 as usize) / 4;
            let y =
                (terminal_size.1 as usize) -
                ((i / (pixels.width as usize)) * (terminal_size.1 as usize)) /
                    (pixels.height as usize);

            let color = Color {
                r,
                g,
                b,
            };

            terminal_fb.set_pixel(x, y, color);
        }
        terminal_fb.draw_frame();
    });
}
