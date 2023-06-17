use std::io::{ BufWriter, Write };

pub struct TerminalFrameBuffer {
    front_buffer: Vec<u32>,
    back_buffer: Vec<u32>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl TerminalFrameBuffer {
    pub fn new(width: usize, height: usize, initial_color: Color) -> TerminalFrameBuffer {
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

    pub fn clear_terminal_and_fill_with_initial_color(&self, initial_color: Color) {
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
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
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

    pub fn draw_frame(&mut self) {
        // let characters = vec!["\u{a0}", ".", ",", ":", ";", "+", "*", "?", "%", "S", "#", "@"];
        //reversed
        let characters = vec!["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", ".", "\u{a0}"];
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
                    let (h, s, l) = rgb_to_hsl(
                        (r as f32) / 255.0,
                        (g as f32) / 255.0,
                        (b as f32) / 255.0
                    );

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

    pub fn clear(&mut self) {
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