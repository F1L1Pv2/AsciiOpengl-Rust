use super::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
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
