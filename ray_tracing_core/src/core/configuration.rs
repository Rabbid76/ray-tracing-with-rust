use crate::core::object::Object;

#[derive(Clone)]
pub struct Configuration {
    pub id: usize,
    pub maximum_depth: usize,
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration {
            id: Object::new_id(),
            maximum_depth: 50,
        }
    }

    pub fn new(maximum_depth: usize) -> Configuration {
        Configuration {
            id: Object::new_id(),
            maximum_depth,
        }
    }
}
