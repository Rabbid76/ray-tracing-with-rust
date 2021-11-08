use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::geometry::instancing;
use ray_tracing_core::geometry::Geometry;
use ray_tracing_core::types::FSize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RotateZ {
    pub id: IdConstructor,
    pub angle: Value,
    pub node: IdReference,
}

impl RotateZ {
    pub fn from_geometry(i: &instancing::RotateZ) -> Result<RotateZ, Box<dyn Error>> {
        Ok(RotateZ {
            id: IdConstructor::Single(i.id),
            angle: Value::from_value(FSize::atan2(i.sin_theta, i.cos_theta).to_degrees())?,
            node: IdReference::Single(i.node.get_id()),
        })
    }

    pub fn to_geometry(
        &self,
        index: usize,
        node: Arc<dyn Geometry>,
    ) -> Result<instancing::RotateZ, Box<dyn Error>> {
        Ok(instancing::RotateZ::new_id(
            self.id.get_id(index),
            self.angle.to_value()?.to_radians(),
            node,
        ))
    }
}

#[cfg(test)]
mod rotate_z_test {
    use super::*;
    use ray_tracing_core::geometry::shape;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Point3;

    #[test]
    fn rotate_z_test_from_rotate_z() {
        let s = shape::Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(material::NoMaterial::new()),
        );
        let s_id = s.id;
        let i = instancing::RotateZ::new(FSize::to_radians(30.0), Arc::new(s));
        let n = RotateZ::from_geometry(&i).unwrap();
        test::assert_eq_float(n.angle.to_value().unwrap(), 30.0, 0.001);
        assert_eq!(n.node, IdReference::Single(s_id));
    }

    #[test]
    fn rotate_z_test_to_rotate_z() {
        let f = RotateZ {
            id: IdConstructor::Single(0),
            angle: Value::Scalar(30.0),
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
        test::assert_eq_float(i.sin_theta, FSize::to_radians(30.0).sin(), 0.001);
        test::assert_eq_float(i.cos_theta, FSize::to_radians(30.0).cos(), 0.001);
    }
}
