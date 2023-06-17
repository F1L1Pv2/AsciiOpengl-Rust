//this file is for importing objects into the engine

use super::object::Object;
use super::matrices::identity_matrix;

#[derive(Clone, Debug)]
pub struct Prefab {
    pub name: String,
    pub model_path: String,
}

pub struct PrefabList {
    pub prefabs: Vec<Prefab>,
}

impl PrefabList {

    pub fn get_prefab(&mut self, name: String) -> Option<Prefab> {
        for prefab in &self.prefabs {
            if prefab.name == name {
                return Some(prefab.clone());
            }
        }
        None
    }

    pub fn load_obj(&mut self, display: &glium::Display, name: &str, model: Option<[[f32; 4]; 4]>) -> Object {
        let load_prefab = self.get_prefab(name.to_string()).unwrap();

        let model = match model {
            Some(model) => model,
            None => identity_matrix(),
        };

        let object = Object::new(
            load_prefab.name,
            model,
            load_prefab.model_path.as_str(),
            display
        );

        object  
    }
}

pub fn get_prefabs() -> PrefabList {
    //get all objects from the models folder

    let mut prefabs = Vec::new();

    //get all file paths
    for entry in std::fs::read_dir("./src/models").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();

        //name is the last part of the path
        let name = path_str.split('/').last().unwrap().to_string();

        prefabs.push(Prefab {
            name,
            model_path: path_str,
        });
    }

    PrefabList {
        prefabs,
    }
}
