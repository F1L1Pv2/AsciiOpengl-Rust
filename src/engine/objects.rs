//WIP

use tobj;
use glium::{self, backend::{glutin, Facade}};

#[derive(Copy, Clone,Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);


#[derive(Debug)]
pub struct Object{
    pub model : [[f32; 4]; 4],
    pub vb : glium::VertexBuffer<Vertex>,
    pub ib : glium::IndexBuffer<u32>,
}

impl Object {
    //load .obj file
    pub fn new(model: [[f32; 4]; 4], file_path: &str, display: &glium::Display) -> Object {

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
            model,
            vb,
            ib,
        }
    }

}