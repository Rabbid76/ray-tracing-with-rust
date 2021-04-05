use crate::serialization::RayTracingObject;
use ray_tracing_core::environment;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

mod sky;
pub use self::sky::*;

pub struct SerializeEnvironment {
    pub object_map: Rc<RefCell<HashMap<usize, RayTracingObject>>>,
}

impl environment::Visitor for SerializeEnvironment {
    fn visit_sky(&mut self, s: &environment::Sky) -> Result<(), Box<dyn Error>> {
        self.object_map
            .borrow_mut()
            .insert(s.id, RayTracingObject::Sky(Sky::from_environment(s)?));
        Ok(())
    }
}

#[cfg(test)]
mod serialize_environment_test {
    use super::*;
    use ray_tracing_core::environment::Environment;
    use ray_tracing_core::types::ColorRGB;

    #[test]
    fn visit_constant_texture_test() {
        let mut s = SerializeEnvironment {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let sk = environment::Sky::new(ColorRGB::new(0.0, 0.0, 0.0), ColorRGB::new(1.0, 1.0, 1.0));
        sk.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 1);
        match &s.object_map.borrow_mut()[&sk.id] {
            RayTracingObject::Sky(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }
}
