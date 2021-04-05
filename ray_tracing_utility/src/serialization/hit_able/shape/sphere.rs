use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::hit_able::shape;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sphere {
    pub id: IdConstructor,
    pub center: Value,
    pub radius: Value,
    pub material: IdReference,
}

impl Sphere {
    pub fn from_shape(s: &shape::Sphere) -> Result<Sphere, Box<dyn Error>> {
        Ok(Sphere {
            id: IdConstructor::Single(s.id),
            center: Value::from_point3(s.center)?,
            radius: Value::from_value(s.radius)?,
            material: IdReference::Single(s.material.get_id()),
        })
    }

    pub fn to_shape(
        &self,
        index: usize,
        material: Arc<dyn Material>,
    ) -> Result<shape::Sphere, Box<dyn Error>> {
        Ok(shape::Sphere {
            id: self.id.get_id(index),
            center: self.center.to_point3()?,
            radius: self.radius.to_value()?,
            material,
        })
    }
}

#[cfg(test)]
mod sphere_test {
    use super::*;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Point3;

    #[test]
    fn sphere_test_from_sphere() {
        let m = Arc::new(material::NoMaterial::new());
        let m_id = m.id;
        let sp = shape::Sphere::new(Point3::new(1.0, 2.0, 3.0), 4.0, m.clone());
        let l = Sphere::from_shape(&sp).unwrap();
        assert_eq!(l.center, Value::Vector3((1.0, 2.0, 3.0)));
        assert_eq!(l.radius, Value::Scalar(4.0));
        assert_eq!(l.material, IdReference::Single(m_id));
    }

    #[test]
    fn sphere_test_to_sphere() {
        let h = Sphere {
            id: IdConstructor::Single(0),
            center: Value::Vector3((1.0, 2.0, 3.0)),
            radius: Value::Scalar(4.0),
            material: IdReference::Single(1),
        };
        let s = h
            .to_shape(0, Arc::new(material::NoMaterial::new()))
            .unwrap();
        test::assert_eq_float(s.radius, 4.0, 0.001);
        test::assert_eq_vector3(&s.center, &Point3::new(1.0, 2.0, 3.0), 0.001);
    }
}
