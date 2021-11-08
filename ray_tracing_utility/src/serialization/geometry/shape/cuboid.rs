use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::geometry::shape;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Cuboid {
    pub id: IdConstructor,
    pub min: Value,
    pub max: Value,
    pub material: IdReference,
}

impl Cuboid {
    pub fn from_shape(c: &shape::Cuboid) -> Result<Cuboid, Box<dyn Error>> {
        Ok(Cuboid {
            id: IdConstructor::Single(c.id),
            min: Value::from_vector3(c.aabb.min)?,
            max: Value::from_vector3(c.aabb.max)?,
            material: IdReference::Single(c.material.get_id()),
        })
    }

    pub fn to_shape(
        &self,
        index: usize,
        material: Arc<dyn Material>,
    ) -> Result<shape::Cuboid, Box<dyn Error>> {
        Ok(shape::Cuboid::new_id(
            self.id.get_id(index),
            self.min.to_vector3()?..self.max.to_vector3()?,
            material,
        ))
    }
}

#[cfg(test)]
mod cuboid_test {
    use super::*;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Point3;

    #[test]
    fn cuboid_test_from_cuboid() {
        let m = Arc::new(material::NoMaterial::new());
        let m_id = m.id;
        let cs = shape::Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            m.clone(),
        );
        let c = Cuboid::from_shape(&cs).unwrap();
        assert_eq!(c.min, Value::Vector3((-1.0, -1.0, -1.0)));
        assert_eq!(c.max, Value::Vector3((1.0, 1.0, 1.0)));
        assert_eq!(c.material, IdReference::Single(m_id));
    }

    #[test]
    fn cuboid_test_to_cuboid() {
        let c = Cuboid {
            id: IdConstructor::Single(0),
            min: Value::Vector3((-1.0, -1.0, -1.0)),
            max: Value::Vector3((1.0, 1.0, 1.0)),
            material: IdReference::Single(1),
        };
        let cs = c
            .to_shape(0, Arc::new(material::NoMaterial::new()))
            .unwrap();
        test::assert_eq_vector3(&cs.aabb.min, &Point3::new(-1.0, -1.0, -1.0), 0.001);
        test::assert_eq_vector3(&cs.aabb.max, &Point3::new(1.0, 1.0, 1.0), 0.001);
    }
}
