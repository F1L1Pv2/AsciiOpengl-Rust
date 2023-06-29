
use super::object::Object;
use serde_json::{Result, Value};

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn load_from_json(filepath: &str, assets_path: &str, display: &glium::Display) -> Result<Scene> {
        let json = std::fs::read_to_string((assets_path.to_owned()+filepath).as_str()).unwrap();
        let v: Value = serde_json::from_str(json.as_str())?;
        let mut scene = Scene::new();

        /*
        
        [
    {
        "Cube": {
            "model_path": "/models/Cube.obj",
            "texture_path": "/sprites/exampletexture.png",
            "model_matrix": {
                "position": [
                    0.0,
                    -2.967487335205078,
                    0.0
                ],
                "rotation": [
                    0.0,
                    0.0,
                    0.0
                ],
                "scale": [
                    1.0,
                    1.0,
                    1.0
                ]
            },
            "tags": [
                "cube",
                "ballzy",
                "ball"
            ]
        }
    },
    {
        "Suzanne": {
            "model_path": "/models/Suzanne.obj",
            "texture_path": null,
            "model_matrix": {
                "position": [
                    0.0,
                    2.6077077388763428,
                    0.0
                ],
                "rotation": [
                    0.0,
                    0.0,
                    0.0
                ],
                "scale": [
                    1.0,
                    1.0,
                    1.0
                ]
            },
            "tags": []
        }
    }
]
        
         */

        for object in v.as_array().unwrap().iter() {
            let object = object.as_object().unwrap();

            let object_name = object.keys().next().unwrap();

            let object = object.get(object_name).unwrap();

            let model_path = object.get("model_path").unwrap().as_str().unwrap();

            let texture_path = object.get("texture_path").unwrap().as_str();

            let model_matrix = object.get("model_matrix").unwrap();

            let position = model_matrix.get("position").unwrap().as_array().unwrap();

            let rotation = model_matrix.get("rotation").unwrap().as_array().unwrap();

            let scale = model_matrix.get("scale").unwrap().as_array().unwrap();

            let tags = object.get("tags").unwrap().as_array().unwrap();

            let mut tags_vec: Vec<String> = Vec::new();

            for tag in tags.iter() {
                tags_vec.push(tag.as_str().unwrap().to_string());
            }

            let texture_path = match texture_path {
                Some(texture_path) => {

                    Some(assets_path.to_owned()+texture_path)
                },
                None => None,
            };

            let texture_path: Option<&str> = texture_path.as_ref().map(|s| &s[..]);

            // println!("Rotation: {:?}", rotation);

            scene.add_object(Object::new(
                (assets_path.to_owned()+model_path).as_str(),
                texture_path,
                super::object::TextureFilter::Linear,
                super::matrices::model_matrix(
                    &[
                        -position[0].as_f64().unwrap() as f32,
                        position[2].as_f64().unwrap() as f32,
                        -position[1].as_f64().unwrap() as f32,
                    ],
                    &[
                        -rotation[0].as_f64().unwrap() as f32,
                        rotation[2].as_f64().unwrap() as f32,
                        -rotation[1].as_f64().unwrap() as f32,
                    ],
                    &[
                        scale[0].as_f64().unwrap() as f32,
                        scale[1].as_f64().unwrap() as f32,
                        scale[2].as_f64().unwrap() as f32,
                    ],
                ),
                display,
                tags_vec,
            ));

        }

        Ok(scene)

    }

    pub fn get_objects_by_tags(&self, tags: Vec<&str>) -> Vec<&Object> {
        //no duplicates
        let mut objects: Vec<&Object> = Vec::new();

        for object in self.objects.iter() {
            for tag in tags.iter() {
                if object.tags.contains(&tag.to_string()) {
                    objects.push(object);
                    break;
                }
            }
        }

        objects
    }

    /// Adds an object to the scene
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
