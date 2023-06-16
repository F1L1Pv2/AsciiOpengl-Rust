use glium::glutin::event;

#[macro_use]
extern crate glium;

mod teapot;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // display.gl_window().window().set_cursor_visible(false);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut player_pos = [0.0, 0.0, 0.0f32];
    let mut player_rot = [0.0, 0.0, 0.0f32];

    let move_speed = 0.05;

    let mouse_sensitive = 0.001;

    let mut move_forward = false;
    let mut move_backward = false;
    let mut move_left = false;
    let mut move_right = false;
    let mut move_up = false;
    let mut move_down = false;

    let mut accumulator = std::time::Duration::new(0, 0);
    let fixed_timestep = std::time::Duration::from_nanos(16_666_667);
    let mut next_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },

            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    let delta = (delta.0 as f32 * mouse_sensitive, delta.1 as f32 * mouse_sensitive);
                    player_rot[0] += delta.1;
                    player_rot[1] += delta.0;
                }

                //keyboard
                glutin::event::DeviceEvent::Key(input) => match input.virtual_keycode {
                    Some(glutin::event::VirtualKeyCode::W) => {
                        move_forward = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::S) => {
                        move_backward = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::A) => {
                        move_left = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::D) => {
                        move_right = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::Escape) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    Some(glutin::event::VirtualKeyCode::Space) => {
                        move_up = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::LShift) => {
                        move_down = input.state == glutin::event::ElementState::Pressed
                    }
                    Some(glutin::event::VirtualKeyCode::Escape) => {
                        //set mouse visible
                        display.gl_window().window().set_cursor_visible(true);
                        

                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    
                    _ => (),
                },

                _ => return,
            },

            glutin::event::Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                accumulator += now - next_frame_time;
                next_frame_time = now;

                while accumulator >= fixed_timestep {
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

                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                let model = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32],
                ];

                let view = view_matrix(&player_pos, &player_rot);

                let perspective = {
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;

                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;

                    let f = 1.0 / (fov / 2.0).tan();

                    [
                        [f * aspect_ratio, 0.0, 0.0, 0.0],
                        [0.0, f, 0.0, 0.0],
                        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                    ]
                };

                let light = [-1.0, 0.4, 0.9f32];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    ..Default::default()
                };

                target
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                        &params,
                    )
                    .unwrap();
                target.finish().unwrap();
            },

            _ => return,
        }
    });
}



fn rotate_x(angle: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin(), 0.0],
        [0.0, angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn rotate_y(angle: f32) -> [[f32; 4]; 4] {
    [
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn rotate_z(angle: f32) -> [[f32; 4]; 4] {
    [
        [angle.cos(), -angle.sin(), 0.0, 0.0],
        [angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0f32, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn translate(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0f32, 0.0],
        [x, y, z, 1.0f32],
    ]
}

//create macro for matrix multiplication
macro_rules! mat_mul {
    ($a:expr, $b:expr) => {{
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += $a[i][k] * $b[k][j];
                }
            }
        }
        result
    }};
}

fn view_matrix(position: &[f32; 3], rotation: &[f32; 3]) -> [[f32; 4]; 4] {
    // let f = rotation;
    // let s = [
    //     f[1] * up[2] - f[2] * up[1],
    //     f[2] * up[0] - f[0] * up[2],
    //     f[0] * up[1] - f[1] * up[0],
    // ];
    // let s_norm = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
    // let s = [s[0] / s_norm, s[1] / s_norm, s[2] / s_norm];
    // let u = [
    //     s[1] * f[2] - s[2] * f[1],
    //     s[2] * f[0] - s[0] * f[2],
    //     s[0] * f[1] - s[1] * f[0],
    // ];
    // let p = [
    //     -(s[0] * position[0] + s[1] * position[1] + s[2] * position[2]),
    //     -(u[0] * position[0] + u[1] * position[1] + u[2] * position[2]),
    //     -(f[0] * position[0] + f[1] * position[1] + f[2] * position[2]),
    // ];
    // [
    //     [s[0], u[0], f[0], 0.0],
    //     [s[1], u[1], f[1], 0.0],
    //     [s[2], u[2], f[2], 0.0],
    //     [p[0], p[1], p[2], 1.0],
    // ]

    let mut m = [
        [1.0, 0.0, 0.0, 0.0f32],
        [0.0, 1.0, 0.0, 0.0f32],
        [0.0, 0.0, 1.0, 0.0f32],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    // let m = (rotate_x(rotation[0]) * m).into();
    // let m = (rotate_y(rotation[1]) * m).into();
    // let m = (rotate_z(rotation[2]) * m).into();
    // let m = (translate(-position[0], -position[1], -position[2]) * m).into();

    m = mat_mul!(rotate_x(rotation[0]), m);
    m = mat_mul!(rotate_y(rotation[1]), m);
    m = mat_mul!(rotate_z(rotation[2]), m);
    m = mat_mul!(translate(-position[0], -position[1], -position[2]), m);

    m
}
