use std::path::Path;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use super::object::Object;
use super::object::Vertex;

pub fn draw_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture_path: &str,
    display: &glium::Display
) -> Object {
    let image = image::open(Path::new(texture_path)).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = Texture2d::new(display, image).unwrap();

    let height = height*2.0;
    
    //currently y is in bottom left corner, need to convert to top left
    let y = 2.0 - (y*2.0) - height;





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

    let ib = glium::IndexBuffer
        ::new(display, glium::index::PrimitiveType::TriangleFan, &indices)
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
        vb,
        ib,
    }
}
