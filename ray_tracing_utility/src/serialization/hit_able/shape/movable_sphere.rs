use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::hit_able::shape;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MovableSphere {
    pub id: IdConstructor,
    pub center0: Value,
    pub center1: Value,
    pub time0: Value,
    pub time1: Value,
    pub radius: Value,
    pub material: IdReference,
}

impl MovableSphere {
    pub fn from_shape(s: &shape::MovableSphere) -> Result<MovableSphere, Box<dyn Error>> {
        Ok(MovableSphere {
            id: IdConstructor::Single(s.id),
            center0: Value::from_point3(s.center_range.start)?,
            center1: Value::from_point3(s.center_range.end)?,
            time0: Value::from_value(s.time_range.start)?,
            time1: Value::from_value(s.time_range.end)?,
            radius: Value::from_value(s.radius)?,
            material: IdReference::Single(s.material.get_id()),
        })
    }

    pub fn to_shape(
        &self,
        index: usize,
        material: Arc<dyn Material>,
    ) -> Result<shape::MovableSphere, Box<dyn Error>> {
        Ok(shape::MovableSphere {
            id: self.id.get_id(index),
            center_range: self.center0.to_point3()?..self.center1.to_point3()?,
            time_range: self.time0.to_value()?..self.time1.to_value()?,
            radius: self.radius.to_value()?,
            material,
        })
    }
}

#[cfg(test)]
mod movable_sphere_test {
    use super::*;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Point3;

    #[test]
    fn movable_sphere_test_from_sphere() {
        let m = Arc::new(material::NoMaterial::new());
        let m_id = m.id;
        let sp = shape::MovableSphere::new(
            Point3::new(1.0, 1.0, 1.0)..Point3::new(2.0, 2.0, 2.0),
            0.0..1.0,
            4.0,
            m.clone(),
        );
        let l = MovableSphere::from_shape(&sp).unwrap();
        assert_eq!(l.center0, Value::Vector3((1.0, 1.0, 1.0)));
        assert_eq!(l.center1, Value::Vector3((2.0, 2.0, 2.0)));
        assert_eq!(l.time0, Value::Scalar(0.0));
        assert_eq!(l.time1, Value::Scalar(1.0));
        assert_eq!(l.radius, Value::Scalar(4.0));
        assert_eq!(l.material, IdReference::Single(m_id));
    }

    #[test]
    fn movable_sphere_test_to_sphere() {
        let h = MovableSphere {
            id: IdConstructor::Single(0),
            center0: Value::Vector3((1.0, 1.0, 1.0)),
            center1: Value::Vector3((2.0, 2.0, 2.0)),
            time0: Value::Scalar(0.0),
            time1: Value::Scalar(1.0),
            radius: Value::Scalar(4.0),
            material: IdReference::Single(1),
        };
        let s = h
            .to_shape(0, Arc::new(material::NoMaterial::new()))
            .unwrap();
        test::assert_eq_float(s.radius, 4.0, 0.001);
        test::assert_eq_vector3(&s.center_range.start, &Point3::new(1.0, 1.0, 1.0), 0.001);
        test::assert_eq_vector3(&s.center_range.end, &Point3::new(2.0, 2.0, 2.0), 0.001);
        test::assert_eq_float(s.time_range.start, 0.0, 0.001);
        test::assert_eq_float(s.time_range.end, 1.0, 0.001);
    }
}
