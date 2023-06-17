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
use engine::ascii_render::{ TerminalFrameBuffer, Color };
use engine::matrices::{ perspective_matrix, view_matrix };
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
    let vertex_shader_src = std::fs::read_to_string("src/shaders/vertex_shader.glsl")
        .expect("Failed to read vertex shader source code from file");

    let fragment_shader_src = std::fs::read_to_string("src/shaders/fragment_shader.glsl")
        .expect("Failed to read fragment shader source code from file");

    let program = glium::Program
        ::from_source(&display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None)
        .unwrap();

    let mut player_pos = [0.0, 0.0, 0.0f32];
    let mut player_rot = [0.0, 0.0, 0.0f32];

    let move_speed = 0.05;

    // let mouse_sensitive = 0.001;

    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

    let device_state = DeviceState::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        let mut move_forward = false;
        let mut move_backward = false;
        let mut move_left = false;
        let mut move_right = false;
        let mut move_up = false;
        let mut move_down = false;

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

        match event {
            glutin::event::Event::WindowEvent { event, .. } =>
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => {
                        return;
                    }
                }

            glutin::event::Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                accumulator += now - next_frame_time;
                next_frame_time = now;

                while accumulator >= fixed_timestep {
                    let keys: Vec<Keycode> = device_state.get_keys();

                    for key in keys {
                        match key {
                            Keycode::W => {
                                move_forward = true;
                            }
                            Keycode::S => {
                                move_backward = true;
                            }
                            Keycode::A => {
                                move_left = true;
                            }
                            Keycode::D => {
                                move_right = true;
                            }
                            Keycode::Space => {
                                move_up = true;
                            }
                            Keycode::LShift => {
                                move_down = true;
                            }
                            Keycode::I => {
                                player_rot[0] -= 0.05;
                            }
                            Keycode::K => {
                                player_rot[0] += 0.05;
                            }
                            Keycode::J => {
                                player_rot[1] -= 0.05;
                            }
                            Keycode::L => {
                                player_rot[1] += 0.05;
                            }
                            _ => (),
                        }
                    }

                    accumulator -= fixed_timestep;

                    // Update player position and rotation
                    if move_forward {
                        player_pos[0] += player_rot[1].sin() * move_speed;
                        player_pos[2] += player_rot[1].cos() * move_speed;
                    }

                    if move_backward {
                        player_pos[0] -= player_rot[1].sin() * move_speed;
                        player_pos[2] -= player_rot[1].cos() * move_speed;
                    }

                    if move_left {
                        player_pos[0] -= player_rot[1].cos() * move_speed;
                        player_pos[2] += player_rot[1].sin() * move_speed;
                    }

                    if move_right {
                        player_pos[0] += player_rot[1].cos() * move_speed;
                        player_pos[2] -= player_rot[1].sin() * move_speed;
                    }

                    if move_up {
                        player_pos[1] += move_speed;
                    }

                    if move_down {
                        player_pos[1] -= move_speed;
                    }
                }

                // framebuffer.clear_color_and_depth((105./255., 109./255., 219./255., 1.0), 1.0);
                framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                let model = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32],
                ];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    ..Default::default()
                };

                let uniforms = uniform! {
                    model: model,
                    view: view_matrix(&player_pos, &player_rot),
                    perspective: perspective_matrix(terminal_size),
                    u_light: [-1.0, 0.4, 0.9f32],
                };

                // target
                framebuffer
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &uniforms,
                        &params
                    )
                    .unwrap();

                // target.finish().unwrap();
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
