//WIP


use glium::texture::RawImage2d;
use glium::{ self, backend::Facade, Texture2d };
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
pub struct AABB {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug)]
pub struct Object {
    // pub name: String,
    pub model: [[f32; 4]; 4],
    pub texture: Texture2d,
    pub texture_filter: TextureFilter,
    pub vb: glium::VertexBuffer<Vertex>,
    pub vertices: Vec<Vertex>,
    pub ib: glium::IndexBuffer<u32>,
    pub tags: Vec<String>,
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
        tags: Vec<String>,
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
                    normal: [mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2]],
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
                }
            );
        }

        // println!("verticies: {:?}", verticies);
        // println!("indicies: {:?}", indicies);

        let vb = glium::VertexBuffer::new(facade, &verticies).unwrap();
        let ib = glium::IndexBuffer
            ::new(facade, glium::index::PrimitiveType::TrianglesList, &indicies)
            .unwrap();


        Object {
            // name,
            model,
            texture: texture_val,
            texture_filter,
            vb,
            vertices: verticies,
            ib,
            tags,
        }
    }

    pub fn get_aabb(&self) -> AABB {

        let vertices = &self.vertices;

        let mut min = [
            10000.0,
            10000.0,
            10000.0,
        ];

        let mut max = [
            -10000.0,
            -10000.0,
            -10000.0,
        ];

        for vertex in vertices.iter(){
            //transform vertex by model matrix
            let mut pos = [
                vertex.position.0,
                vertex.position.1,
                vertex.position.2,
                1.0,
            ];

            pos = [
                pos[0] * self.model[0][0] + pos[1] * self.model[0][1] + pos[2] * self.model[0][2] + pos[3] * self.model[0][3],
                pos[0] * self.model[1][0] + pos[1] * self.model[1][1] + pos[2] * self.model[1][2] + pos[3] * self.model[1][3],
                pos[0] * self.model[2][0] + pos[1] * self.model[2][1] + pos[2] * self.model[2][2] + pos[3] * self.model[2][3],
                pos[0] * self.model[3][0] + pos[1] * self.model[3][1] + pos[2] * self.model[3][2] + pos[3] * self.model[3][3],
            ];


            //translate pos by model matrix
            pos = [
                pos[0] + self.model[3][0],
                pos[1] + self.model[3][1],
                pos[2] + self.model[3][2],
                pos[3] + self.model[3][3],
            ];

            if pos[0] < min[0] {
                min[0] = pos[0];
            }
            if pos[1] < min[1] {
                min[1] = pos[1];
            }
            if pos[2] < min[2] {
                min[2] = pos[2];
            }

            if pos[0] > max[0] {
                max[0] = pos[0];
            }
            if pos[1] > max[1] {
                max[1] = pos[1];
            }
            if pos[2] > max[2] {
                max[2] = pos[2];
            }


        }

        AABB {
            min,
            max,
        }
        
    }

    pub fn get_aabb_acc(&self, acc: (f32, f32, f32)) -> AABB {

        let vertices = &self.vertices;

        let mut min = [
            10000.0,
            10000.0,
            10000.0,
        ];

        let mut max = [
            -10000.0,
            -10000.0,
            -10000.0,
        ];

        for vertex in vertices.iter(){
            //transform vertex by model matrix
            let mut pos = [
                vertex.position.0,
                vertex.position.1,
                vertex.position.2,
                1.0,
            ];

            pos = [
                pos[0] * self.model[0][0] + pos[1] * self.model[0][1] + pos[2] * self.model[0][2] + pos[3] * self.model[0][3],
                pos[0] * self.model[1][0] + pos[1] * self.model[1][1] + pos[2] * self.model[1][2] + pos[3] * self.model[1][3],
                pos[0] * self.model[2][0] + pos[1] * self.model[2][1] + pos[2] * self.model[2][2] + pos[3] * self.model[2][3],
                pos[0] * self.model[3][0] + pos[1] * self.model[3][1] + pos[2] * self.model[3][2] + pos[3] * self.model[3][3],
            ];


            //translate pos by model matrix
            pos = [
                pos[0] + self.model[3][0] + acc.0,
                pos[1] + self.model[3][1] + acc.1,
                pos[2] + self.model[3][2] + acc.2,
                pos[3] + self.model[3][3],
            ];

            if pos[0] < min[0] {
                min[0] = pos[0];
            }
            if pos[1] < min[1] {
                min[1] = pos[1];
            }
            if pos[2] < min[2] {
                min[2] = pos[2];
            }

            if pos[0] > max[0] {
                max[0] = pos[0];
            }
            if pos[1] > max[1] {
                max[1] = pos[1];
            }
            if pos[2] > max[2] {
                max[2] = pos[2];
            }


        }

        AABB {
            min,
            max,
        }
        
    }

    pub fn check_aabb_collision(&self, other: &Object) -> bool {
        let a = self.get_aabb();
        let b = other.get_aabb();

        a.min[0] <= b.max[0] &&
            a.max[0] >= b.min[0] &&
            a.min[1] <= b.max[1] &&
            a.max[1] >= b.min[1] &&
            a.min[2] <= b.max[2] &&
            a.max[2] >= b.min[2]
    }

    pub fn check_aabb_collision_acc(&self, acc: (f32,f32,f32), other: &Object) -> bool {
        let a = self.get_aabb_acc(acc);
        let b = other.get_aabb();

        a.min[0] <= b.max[0] &&
            a.max[0] >= b.min[0] &&
            a.min[1] <= b.max[1] &&
            a.max[1] >= b.min[1] &&
            a.min[2] <= b.max[2] &&
            a.max[2] >= b.min[2]
    }
}
