#[macro_use]
extern crate glium;

use std::io::{ BufWriter, Write };

use device_query;
use device_query::Keycode;
use device_query::DeviceState;
use device_query::DeviceQuery;

use glium::debug;
use termion;

mod teapot;
struct TerminalFrameBuffer {
    front_buffer: Vec<u32>,
    back_buffer: Vec<u32>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl TerminalFrameBuffer {
    fn new(width: usize, height: usize, initial_color: Color) -> TerminalFrameBuffer {
        let initial_color_value =
            (u32::from(initial_color.r) << 16) |
            (u32::from(initial_color.g) << 8) |
            u32::from(initial_color.b);
        let framebuffer = TerminalFrameBuffer {
            front_buffer: vec![initial_color_value; width * height],
            back_buffer: vec![initial_color_value; width * height],
            width,
            height,
        };
        framebuffer.clear_terminal_and_fill_with_initial_color(initial_color);
        framebuffer
    }

    fn clear_terminal_and_fill_with_initial_color(&self, initial_color: Color) {
        let stdout = std::io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        write!(out, "\x1B[2J\x1B[1;1H").unwrap();
        for _y in 0..self.height {
            for _x in 0..self.width {
                write!(
                    out,
                    "\x1b[48;2;{};{};{}m  ",
                    initial_color.r,
                    initial_color.g,
                    initial_color.b
                ).unwrap();
            }
            writeln!(out, "\x1b[0m").unwrap();
        }
        out.flush().unwrap();
    }
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let color = (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b);
        //check if x and y are in bounds
        if x >= self.width || y >= self.height {
            return;
        }
        self.back_buffer[y * self.width + x] = color;
    }

    fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.front_buffer[y * self.width + x]
    }

    fn draw_frame(&mut self) {
        let characters = vec![" ", ".", ",", ":", ";", "+", "*", "?", "%", "S", "#", "@"];

        let stdout = std::io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        for y in 0..self.height {
            for x in 0..self.width {
                let front_pixel = self.get_pixel(x, y);
                let back_pixel = self.back_buffer[y * self.width + x];
                if front_pixel != back_pixel {
                    let r = (back_pixel >> 16) & 0xff;
                    let g = (back_pixel >> 8) & 0xff;
                    let b = back_pixel & 0xff;

                    //calculate hsl
                    let (h, s, l) = rgb_to_hsl(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

                    //calculate character from l
                    let character_index = ((1.0 - l) * ((characters.len() - 1) as f32)) as usize;
                    let character = characters[character_index];

                    //calculate color from h and s
                    let (fr, fg, fb) = hsl_to_rgb(h, s, 0.5);

                    write!(
                        out,
                        "\x1B[{};{}H\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{}{}",
                        y + 1,
                        x * 2 + 1,
                        r,
                        g,
                        b,
                        fr,
                        fg,
                        fb,
                        character,
                        character
                    ).unwrap();
                }
            }
        }
        write!(out, "\x1B[{};{}H\x1b[0m", self.height + 1, 1).unwrap();
        out.flush().unwrap();
        self.swap_buffers();
    }

    fn clear(&mut self) {
        self.back_buffer = vec![0; self.width * self.height];
    }

    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let cmax = r.max(g.max(b));
    let cmin = r.min(g.min(b));
    let delta = cmax - cmin;

    let h = if delta == 0.0 {
        0.0
    } else if cmax == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if cmax == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let l = (cmax + cmin) / 2.0;

    let s = if delta == 0.0 { 0.0 } else { delta / (1.0 - (2.0 * l - 1.0).abs()) };

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    (r, g, b)
}

fn main() {
    #[allow(unused_imports)]
    use glium::{ glutin, Surface };

    let terminal_size = termion::terminal_size().unwrap();
    let terminal_size = (terminal_size.0 as u32, terminal_size.1 as u32);

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
    // let wb = glutin::window::WindowBuilder::new();

    //offscreen rendering
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

    let vertex_shader_src =
        r#"
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

    let fragment_shader_src =
        r#"
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

    let program = glium::Program
        ::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .unwrap();

    let mut player_pos = [0.0, 0.0, 0.0f32];
    let mut player_rot = [0.0, 0.0, 0.0f32];

    let move_speed = 0.05;

    let mouse_sensitive = 0.001;

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

        let mut texture = glium::texture::Texture2d
            ::empty_with_format(
                &display,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                terminal_size.0,
                terminal_size.1
            )
            .unwrap();

        // Create a depth buffer for off-screen rendering
        let mut depthbuffer = glium::framebuffer::DepthRenderBuffer
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

                framebuffer.clear_color_and_depth((105./255., 109./255., 219./255., 1.0), 1.0);

                let model = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32],
                ];

                let view = view_matrix(&player_pos, &player_rot);

                let perspective = {
                    // let (width, height) = target.get_dimensions();
                    let (width, height) = terminal_size;
                    let aspect_ratio = (height as f32) / (width as f32);

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

                // target
                framebuffer
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &(uniform! { model: model, view: view, perspective: perspective, u_light: light }),
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
        // let pixels: glium::texture::RawImage2d<u8> = display.read_front_buffer().unwrap();
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
                r: r,
                g: g,
                b: b,
            };

            terminal_fb.set_pixel(x, y, color);
        }
        terminal_fb.draw_frame();
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
    ($a:expr, $b:expr) => {
        {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += $a[i][k] * $b[k][j];
                }
            }
        }
        result
        }
    };
}

fn view_matrix(position: &[f32; 3], rotation: &[f32; 3]) -> [[f32; 4]; 4] {
    let mut m = [
        [1.0, 0.0, 0.0, 0.0f32],
        [0.0, 1.0, 0.0, 0.0f32],
        [0.0, 0.0, 1.0, 0.0f32],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    m = mat_mul!(rotate_x(rotation[0]), m);
    m = mat_mul!(rotate_y(rotation[1]), m);
    m = mat_mul!(rotate_z(rotation[2]), m);
    m = mat_mul!(translate(-position[0], -position[1], -position[2]), m);

    m
}
