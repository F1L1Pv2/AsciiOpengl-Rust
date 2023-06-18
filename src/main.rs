#[macro_use]
extern crate glium;
use glium::{ glutin, Surface };

//-------------- Terminal Stuff ------------------------
use device_query::{  DeviceState};
use terminal_size::terminal_size;
//------------------ My stuff --------------------------
mod engine;
use engine::scene::Scene;
use engine::ascii_render::{ Color, TerminalFrameBuffer };
use engine::core::{ init, Game };
use engine::matrices::model_matrix;
use engine::game_loop::game_loop;
// -----------------------------------------------------

fn main() {

    let (
        mut terminal_res,
        mut terminal_fb,
        event_loop,
        display,
        program,
        params,
        mut camera,
        mut prefab_list,
    ) = init();

    let mut game = Game::new();

    let mut scene: Scene = Scene::new();

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "monke.obj",
            model_matrix(&[0.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]).into()
        )
    );

    game.add_scene(scene);

    let mut scene = Scene::new();

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "cube.obj",
            model_matrix(&[-4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]).into()
        )
    );

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "cube.obj",
            model_matrix(&[4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]).into()
        )
    );

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "cube.obj",
            model_matrix(&[0.0, 0.0, 6.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]).into()
        )
    );

    scene.add_object(
        prefab_list.load_obj(
            &display,
            "cube.obj",
            model_matrix(&[0.0, 0.0, -2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]).into()
        )
    );

    game.add_scene(scene);



    drop(prefab_list);

    let light = [1.4, 0.4, -0.7f32];
    let device_state = DeviceState::new();
    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

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


                    game_loop(&device_state, terminal_res, &mut camera, &mut game);
                    
                    accumulator -= fixed_timestep;
                }

                //--------------------------------- Render (post update) ---------------------------------


                framebuffer.clear_color_and_depth(
                    (105.0 / 255.0, 109.0 / 255.0, 219.0 / 255.0, 1.0),
                    1.0
                );

                for object in game.get_scene().objects.iter() {
                    let uniforms =
                        uniform! {
                        model: object.model,
                        view: camera.view_matrix(),
                        perspective: camera.perspective_matrix(),
                        u_light: light,
                    };

                    framebuffer.draw(&object.vb, &object.ib, &program, &uniforms, &params).unwrap();
                }
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
