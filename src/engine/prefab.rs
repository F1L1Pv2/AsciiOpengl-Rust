//this file is for importing objects into the engine

use super::object::Object;
use super::matrices::identity_matrix;

#[derive(Clone, Debug)]
pub struct prefab{
    pub name: String,
    pub model_path: String,
}

pub struct prefab_list {
    pub prefabs: Vec<prefab>,
}

impl prefab_list {
    fn new() -> prefab_list {
        prefab_list {
            prefabs: Vec::new(),
        }
    }

    pub fn get_prefab(&mut self, name: String) -> Option<prefab> {
        for prefab in &self.prefabs {
            if prefab.name == name {
                return Some(prefab.clone());
            }
        }
        None
    }

    pub fn load_object(&mut self, display: &glium::Display, prefab: prefab)-> Object {
        let mut object = Object::new(
            prefab.name,
            identity_matrix(),
            prefab.model_path.as_str(),
            display
        );


        object
    }
}

pub fn get_prefabs(display: &glium::Display) -> prefab_list {

    //get all objects from the models folder
    
    let mut prefabs = Vec::new();
    
    //get all file paths
    for entry in std::fs::read_dir("./src/models").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        
        //name is the last part of the path
        let name = path_str.split("/").last().unwrap().to_string();

        prefabs.push(prefab {
            name,
            model_path: path_str,
        });
    }

    prefab_list {
        prefabs
    }

}