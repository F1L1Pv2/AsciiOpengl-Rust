#[macro_use]
extern crate glium;

//-------------- Terminal Stuff ------------------------
use device_query::{ DeviceQuery, DeviceState, Keycode };
use terminal_size::terminal_size;
//------------------ My stuff --------------------------
mod engine;
use engine::core::{ Scene };
use engine::prefab::{ get_prefabs };
use engine::ascii_render::{ Color, TerminalFrameBuffer };
use engine::camera::Camera;
// -----------------------------------------------------

fn main() {
    use glium::{ glutin, Surface };

    let terminal_res = terminal_size().unwrap();
    let mut terminal_res: (u32, u32) = (u32::from(terminal_res.0.0), u32::from(terminal_res.1.0));

    let mut terminal_fb = TerminalFrameBuffer::new(
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

    let mut camera = Camera::new([0.0, 0.0, 0.0f32], [0.0, 0.0, 0.0f32], 0.05, 0.05, terminal_res);

    let mut prefab_list = get_prefabs();

    let mut scene = Scene::new();

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "monke.obj",
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 2.0, 1.0f32],
            ].into()
        )
    );

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "cube.obj",
            [
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [2.0, 0.0, 2.0, 1.0f32],
            ].into()
        )
    );

    drop(prefab_list);

    let light = [1.4, 0.4, -0.7f32];

    // Main loop
    event_loop.run(move |event, _, control_flow| {
        // Check res and update if changed
        let new_terminal_res = terminal_size().unwrap();
        let new_terminal_res: (u32, u32) = (
            u32::from(new_terminal_res.0.0),
            u32::from(new_terminal_res.1.0),
        );
        if new_terminal_res != terminal_res {
            terminal_res = new_terminal_res;
            terminal_fb = TerminalFrameBuffer::new(
                (terminal_res.0 as usize) / 2,
                terminal_res.1 as usize,
                Color { r: 0, g: 0, b: 0 }
            );
            TerminalFrameBuffer::update_res(
                &mut terminal_fb,
                new_terminal_res.0 as usize,
                new_terminal_res.1 as usize
            );
        }

        *control_flow = glutin::event_loop::ControlFlow::Poll;

        let mut move_vector = [0, 0, 0];
        let mut mouse_vector = [0, 0];

        let texture = glium::texture::Texture2d
            ::empty_with_format(
                &display,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                terminal_res.0,
                terminal_res.1
            )
            .unwrap();

        // Create a depth buffer for off-screen rendering
        let depthbuffer = glium::framebuffer::DepthRenderBuffer
            ::new(&display, glium::texture::DepthFormat::F32, terminal_res.0, terminal_res.1)
            .unwrap();

        // Create a framebuffer for off-screen rendering
        let mut framebuffer: glium::framebuffer::SimpleFrameBuffer = glium::framebuffer::SimpleFrameBuffer
            ::with_depth_buffer(&display, &texture, &depthbuffer)
            .unwrap();

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
                    camera.update(terminal_res, move_vector, mouse_vector);
                    accumulator -= fixed_timestep;
                }

                //--------------------------------- Render (post update) ---------------------------------

                framebuffer.clear_color_and_depth(
                    (105.0 / 255.0, 109.0 / 255.0, 219.0 / 255.0, 1.0),
                    1.0
                );

                for object in scene.objects.iter() {
                    let uniforms =
                        uniform! {
                        model: object.model,
                        view: camera.view_matrix(),
                        perspective: camera.perspective_matrix(),
                        u_light: light,
                    };

                    framebuffer.draw(&object.vb, &object.ib, &program, &uniforms, &params).unwrap();
                }

                // let cube_uniforms = uniform! {
                //     model: cube.model,
                //     view: camera.view_matrix(),
                //     perspective: camera.perspective_matrix(),
                //     u_light: light,
                // };

                // framebuffer
                //     .draw(&cube.vb, &cube.ib, &program, &cube_uniforms, &params)
                //     .unwrap();

                // let monke_uniforms = uniform! {
                //     model: monke.model,
                //     view: camera.view_matrix(),
                //     perspective: camera.perspective_matrix(),
                //     u_light: light,
                // };

                // // target
                // framebuffer
                //     .draw(&monke.vb, &monke.ib, &program, &monke_uniforms, &params)
                //     .unwrap();
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
                ((i % (pixels.width as usize)) * (terminal_res.0 as usize)) /
                    (pixels.width as usize) -
                (terminal_res.0 as usize) / 4;
            let y =
                (terminal_res.1 as usize) -
                ((i / (pixels.width as usize)) * (terminal_res.1 as usize)) /
                    (pixels.height as usize);

            let color = Color { r, g, b };

            terminal_fb.set_pixel(x, y, color);
        }
        terminal_fb.draw_frame();
    });
}
