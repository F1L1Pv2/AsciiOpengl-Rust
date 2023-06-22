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
