use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::geometry::shape;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct YZRect {
    pub id: IdConstructor,
    pub y0: Value,
    pub z0: Value,
    pub y1: Value,
    pub z1: Value,
    pub k: Value,
    pub material: IdReference,
}

impl YZRect {
    pub fn from_shape(r: &shape::YZRect) -> Result<YZRect, Box<dyn Error>> {
        Ok(YZRect {
            id: IdConstructor::Single(r.id),
            y0: Value::from_value(r.rect.start.0)?,
            z0: Value::from_value(r.rect.start.1)?,
            y1: Value::from_value(r.rect.end.0)?,
            z1: Value::from_value(r.rect.end.1)?,
            k: Value::from_value(r.k)?,
            material: IdReference::Single(r.material.get_id()),
        })
    }

    pub fn to_shape(
        &self,
        index: usize,
        material: Arc<dyn Material>,
    ) -> Result<shape::YZRect, Box<dyn Error>> {
        Ok(shape::YZRect {
            id: self.id.get_id(index),
            rect: ((self.y0.to_value()?, self.z0.to_value()?)
                ..(self.y1.to_value()?, self.z1.to_value()?)),
            k: self.k.to_value()?,
            material,
        })
    }
}

#[cfg(test)]
mod yz_rect_test {
    use super::*;
    use ray_tracing_core::material;
    use ray_tracing_core::test;

    #[test]
    fn yz_rect_test_from_yz_rect() {
        let m = Arc::new(material::NoMaterial::new());
        let m_id = m.id;
        let rs = shape::YZRect::new((0.0, 0.0)..(1.0, 1.0), 0.0, m.clone());
        let r = YZRect::from_shape(&rs).unwrap();
        assert_eq!(r.y0, Value::Scalar(0.0));
        assert_eq!(r.z0, Value::Scalar(0.0));
        assert_eq!(r.y1, Value::Scalar(1.0));
        assert_eq!(r.z1, Value::Scalar(1.0));
        assert_eq!(r.k, Value::Scalar(0.0));
        assert_eq!(r.material, IdReference::Single(m_id));
    }

    #[test]
    fn yz_rect_test_to_yz_rect() {
        let r = YZRect {
            id: IdConstructor::Single(0),
            y0: Value::Scalar(0.0),
            z0: Value::Scalar(0.0),
            y1: Value::Scalar(1.0),
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
