use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::hit_able::shape;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct XZRect {
    pub id: IdConstructor,
    pub x0: Value,
    pub z0: Value,
    pub x1: Value,
    pub z1: Value,
    pub k: Value,
    pub material: IdReference,
}

impl XZRect {
    pub fn from_shape(r: &shape::XZRect) -> Result<XZRect, Box<dyn Error>> {
        Ok(XZRect {
            id: IdConstructor::Single(r.id),
            x0: Value::from_value(r.rect.start.0)?,
            z0: Value::from_value(r.rect.start.1)?,
            x1: Value::from_value(r.rect.end.0)?,
            z1: Value::from_value(r.rect.end.1)?,
            k: Value::from_value(r.k)?,
            material: IdReference::Single(r.material.get_id()),
        })
    }

    pub fn to_shape(
        &self,
        index: usize,
        material: Arc<dyn Material>,
    ) -> Result<shape::XZRect, Box<dyn Error>> {
        Ok(shape::XZRect {
            id: self.id.get_id(index),
            rect: ((self.x0.to_value()?, self.z0.to_value()?)
                ..(self.x1.to_value()?, self.z1.to_value()?)),
            k: self.k.to_value()?,
            material,
        })
    }
}

#[cfg(test)]
mod xz_rect_test {
    use super::*;
    use ray_tracing_core::material;
    use ray_tracing_core::test;

    #[test]
    fn xz_rect_test_from_xz_rect() {
        let m = Arc::new(material::NoMaterial::new());
        let m_id = m.id;
        let rs = shape::XZRect::new((0.0, 0.0)..(1.0, 1.0), 0.0, m.clone());
        let r = XZRect::from_shape(&rs).unwrap();
        assert_eq!(r.x0, Value::Scalar(0.0));
        assert_eq!(r.z0, Value::Scalar(0.0));
        assert_eq!(r.x1, Value::Scalar(1.0));
        assert_eq!(r.z1, Value::Scalar(1.0));
        assert_eq!(r.k, Value::Scalar(0.0));
        assert_eq!(r.material, IdReference::Single(m_id));
    }

    #[test]
    fn xz_rect_test_to_xz_rect() {
        let r = XZRect {
            id: IdConstructor::Single(0),
            x0: Value::Scalar(0.0),
            z0: Value::Scalar(0.0),
            x1: Value::Scalar(1.0),
            z1: Value::Scalar(1.0),
            k: Value::Scalar(0.0),
            material: IdReference::Single(1),
        };
        let rs = r
            .to_shape(0, Arc::new(material::NoMaterial::new()))
            .unwrap();
        test::assert_eq_float(rs.rect.start.0, 0.0, 0.001);
        test::assert_eq_float(rs.rect.start.1, 0.0, 0.001);
        test::assert_eq_float(rs.rect.end.0, 1.0, 0.001);
        test::assert_eq_float(rs.rect.end.1, 1.0, 0.001);
        test::assert_eq_float(rs.k, 0.0, 0.001);
    }
}
