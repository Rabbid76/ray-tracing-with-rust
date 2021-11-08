use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::geometry::instancing;
use ray_tracing_core::geometry::Geometry;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Translate {
    pub id: IdConstructor,
    pub offset: Value,
    pub node: IdReference,
}

impl Translate {
    pub fn from_geometry(i: &instancing::Translate) -> Result<Translate, Box<dyn Error>> {
        Ok(Translate {
            id: IdConstructor::Single(i.id),
            offset: Value::from_vector3(i.offset)?,
            node: IdReference::Single(i.node.get_id()),
        })
    }

    pub fn to_geometry(
        &self,
        index: usize,
        node: Arc<dyn Geometry>,
    ) -> Result<instancing::Translate, Box<dyn Error>> {
        Ok(instancing::Translate::new_id(
            self.id.get_id(index),
            self.offset.to_vector3()?,
            node,
        ))
    }
}

#[cfg(test)]
mod translate_test {
    use super::*;
    use ray_tracing_core::geometry::shape;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::{Point3, Vector3};

    #[test]
    fn translate_test_from_translate() {
        let s = shape::Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(material::NoMaterial::new()),
        );
        let s_id = s.id;
        let i = instancing::Translate::new(Vector3::new(0.0, 1.0, 0.0), Arc::new(s));
        let n = Translate::from_geometry(&i).unwrap();
        assert_eq!(n.offset, Value::Vector3((0.0, 1.0, 0.0)));
        assert_eq!(n.node, IdReference::Single(s_id));
    }

    #[test]
    fn translate_test_to_translate() {
        let f = Translate {
            id: IdConstructor::Single(0),
            offset: Value::Vector3((0.0, 1.0, 0.0)),
            node: IdReference::Single(1),
        };
        let i = f
            .to_geometry(
                0,
                Arc::new(shape::Sphere::new(
                    Point3::new(0.0, 0.0, 0.0),
                    1.0,
                    Arc::new(material::NoMaterial::new()),
                )),
            )
            .unwrap();
        test::assert_eq_vector3(&i.offset, &Vector3::new(0.0, 1.0, 0.0), 0.001);
    }
}
