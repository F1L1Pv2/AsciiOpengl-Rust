#[macro_use]
extern crate glium;
use glium::{ glutin, Surface };

//-------------- Terminal Stuff ------------------------
use device_query::{ DeviceState };
use terminal_size::terminal_size;
// -----------------------------------------------------
use fontdue::Font;
//------------------ My stuff --------------------------
mod engine;
use engine::scene::Scene;
use engine::ascii_render::{ Color, TerminalFrameBuffer };
use engine::core::{ init};
use engine::matrices::model_matrix;
use engine::game_loop::game_loop;
use engine::object::Object;
// use engine::ui::draw_rect;
use engine::ui::{draw_text, draw_rect};
// -----------------------------------------------------

fn main() {
    let (
        mut terminal_res,
        mut terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        mut game,
    ) = init();

    let mut scene: Scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/monke.obj",
            None,
            model_matrix(&[0.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    game.add_scene(scene);

    let mut scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            "assets/sprites/align.png".into(),
            model_matrix(&[-4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, 6.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, -2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    game.add_scene(scene);

    let font = Font::from_bytes(
        include_bytes!("../assets/fonts/Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings::default()
    ).unwrap();

    game.add_ui_elems(draw_text(0.0, 0.0, "ballin", 3.0, &font, &display));
    // game.add_ui_elems(draw_text(0.0, 0.5, "b", 20.0, &font, &display));

    // return;


    game.add_ui_elem(draw_rect(0.0, 0.75, 0.25, 0.25, "assets/sprites/align.png", &display));

    // println!("{:?}", game.get_ui_elems());

    // return;

    let light = [1.4, 0.4, -0.7f32];
    let device_state = DeviceState::new();
    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

    // Main loop
    event_loop.run(move |event, _, control_flow| {
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

                    game_loop(&device_state, terminal_res, &mut game);

                    accumulator -= fixed_timestep;
                }

                //--------------------------------- Render (post update) ---------------------------------

                framebuffer.clear_color_and_depth(
                    (105.0 / 255.0, 109.0 / 255.0, 219.0 / 255.0, 1.0),
                    1.0
                );

                for object in &game.get_scene().objects {
                    let uniforms =
                        uniform! {
                        model: object.model,
                        view: game.camera.view_matrix(),
                        perspective: game.camera.perspective_matrix(),
                        u_light: light,
                        tex: &object.texture,
                    };

                    framebuffer.draw(&object.vb, &object.ib, &program, &uniforms, &params).unwrap();
                }

                //--------------------------------- UI ---------------------------------

                for ui_elem in &game.get_ui_elems().elems {

                    // let behavior = glium::uniforms::SamplerBehavior {
                    //     minify_filter:  glium::uniforms::MinifySamplerFilter::Nearest,
                    //     magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
                    //     ..Default::default()
                    // };

                    let uniforms =
                        uniform! {
                        // tex: glium::uniforms::Sampler(&ui_elem.texture, behavior),
                        tex: &ui_elem.texture,
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
