use super::object::Vertex;
use super::object::{Object, TextureFilter};
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use std::path::Path;

use fontdue::Font;
use image::ImageBuffer;

#[allow(dead_code)]
/// writes text to the screen at the given origin with the given size and font
pub fn draw_text(
    origin_x: f32,
    origin_y: f32,
    text: &str,
    size: f32,
    font: &Font,
    display: &glium::Display,
) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();

    //we are using fontdue to render text
    let mut x_offset = 0.0;

    for (_i, character) in text.chars().enumerate() {
        // println!("character: {} origin_x {} origin_y {}", character, origin_x, origin_y);
        let size = size * 5.0;

        let (metrics, bitmap) = font.rasterize(character, size);
        let (width, height) = (metrics.width as u32, metrics.height as u32);
        let mut image = ImageBuffer::new(width, height);

        for (i, pixel) in image.pixels_mut().enumerate() {
            let alpha = bitmap[i];
            *pixel = image::Rgba([255, 255, 255, alpha]);
        }

        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = Texture2d::new(display, image).unwrap();

        let actual_width = metrics.bounds.width / 100.;
        let actual_height = metrics.bounds.height / 100.;

        let x = origin_x + x_offset;
        let y = origin_y;

        x_offset += actual_width + 0.01;

        let actual_height = actual_height * 2.0;

        let y = 2.0 - y * 2.0 - actual_height;

        let vertices = vec![
            Vertex {
                position: (x, y, 0.0),
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: (x + actual_width, y, 0.0),
                normal: [0.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: (x + actual_width, y + actual_height, 0.0),
                normal: [0.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: (x, y + actual_height, 0.0),
                normal: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let ib =
            glium::IndexBuffer::new(display, glium::index::PrimitiveType::TriangleFan, &indices)
                .unwrap();

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0, 1.0f32],
        ];


        let object = Object {
            vb: glium::VertexBuffer::new(display, &vertices).unwrap(),
            vertices,
            ib,
            texture,
            texture_filter: TextureFilter::Nearest,
            model,
            tags: vec!["ui".to_string()]
        };

        objects.push(object);
    }

    objects
}

#[allow(dead_code)]
/// draws a rectangle with given  texture
pub fn draw_rect_tex(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture: Texture2d,
    texture_filter: TextureFilter,
    display: &glium::Display,
) -> Object {
    let height = height * 2.0;

    let y = 2.0 - y * 2.0 - height;

    let vertices = vec![
        Vertex {
            position: (x, y, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: (x + width, y, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: (x + width, y + height, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: (x, y + height, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    let vb = glium::VertexBuffer::new(display, &vertices).unwrap();

    let ib = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TriangleFan, &indices)
        .unwrap();

    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];


    Object {
        model,
        texture,
        texture_filter,
        vertices,
        vb,
        ib,
        tags: vec!["ui".to_string()]
    }
}

#[allow(dead_code)]
/// draws a rectangle with texture from given path
pub fn draw_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture_path: &str,
    texture_filter: TextureFilter,
    display: &glium::Display,
) -> Object {
    let image = image::open(Path::new(texture_path)).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = Texture2d::new(display, image).unwrap();

    let height = height * 2.0;

    //currently y is in bottom left corner, need to convert to top left
    let y = 2.0 - y * 2.0 - height;

    let vertices = vec![
        Vertex {
            position: (x, y, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: (x + width, y, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: (x + width, y + height, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: (x, y + height, 0.0),
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    let vb = glium::VertexBuffer::new(display, &vertices).unwrap();

    let ib = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TriangleFan, &indices)
        .unwrap();

    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];


    Object {
        model,
        texture,
        texture_filter,
        vertices,
        vb,
        ib,
        tags: vec!["ui".to_string()]
    }
}
