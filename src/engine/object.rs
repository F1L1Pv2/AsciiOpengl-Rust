//WIP

use glium::texture::RawImage2d;
use glium::{self, backend::Facade, Texture2d};
use std::path::Path;
use tobj;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

#[derive(Debug)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

#[derive(Debug)]
pub struct Object {
    // pub name: String,
    pub model: [[f32; 4]; 4],
    pub texture: Texture2d,
    pub texture_filter: TextureFilter,
    pub vb: glium::VertexBuffer<Vertex>,
    pub ib: glium::IndexBuffer<u32>,
}

impl Object {
    //load .obj file
    pub fn new(
        // name: String,
        file_path: &str,
        texture_path: Option<&str>,
        texture_filter: TextureFilter,
        model: [[f32; 4]; 4],
        display: &glium::Display,
    ) -> Object {
        let facade = display.get_context();

        let load_options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        };
        let (models, _materials) = tobj::load_obj(file_path, &load_options).unwrap();
        let mut verticies = Vec::new();
        let mut indicies = Vec::new();

        for m in models {
            let mesh = m.mesh;
            for i in 0..mesh.positions.len() / 3 {
                let vertex = Vertex {
                    position: (
                        mesh.positions[i * 3],
                        mesh.positions[i * 3 + 1],
                        mesh.positions[i * 3 + 2],
                    ),
                    normal: [
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    ],
                    tex_coords: [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]],
                };
                verticies.push(vertex);
            }
            indicies = mesh.indices;
        }

        let texture_val: Texture2d;

        if let Some(texture_path) = texture_path {
            let texture_path = texture_path.to_string();
            let image = image::open(Path::new(&texture_path)).unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            texture_val = Texture2d::new(display, image).unwrap();
        } else {
            //create blank texture

            texture_val = Texture2d::empty(display, 512, 512).unwrap();
            //fill texture with white
            let texture_data = vec![255u8; 512 * 512 * 4];

            texture_val.write(
                glium::Rect {
                    left: 0,
                    bottom: 0,
                    width: 512,
                    height: 512,
                },
                glium::texture::RawImage2d {
                    data: std::borrow::Cow::Borrowed(&texture_data),
                    width: 512,
                    height: 512,
                    format: glium::texture::ClientFormat::U8U8U8U8,
                },
            );
        }

        // println!("verticies: {:?}", verticies);
        // println!("indicies: {:?}", indicies);

        let vb = glium::VertexBuffer::new(facade, &verticies).unwrap();
        let ib = glium::IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TrianglesList,
            &indicies,
        )
        .unwrap();

        Object {
            // name,
            model,
            texture: texture_val,
            texture_filter,
            vb,
            ib,
        }
    }
}
