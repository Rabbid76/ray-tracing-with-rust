use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::geometry::volume;
use ray_tracing_core::geometry::Geometry;
use ray_tracing_core::material::Material;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConstantMedium {
    pub id: IdConstructor,
    pub density: Value,
    pub boundary: IdReference,
    pub phase_function: IdReference,
}

impl ConstantMedium {
    pub fn from_volume(i: &volume::ConstantMedium) -> Result<ConstantMedium, Box<dyn Error>> {
        Ok(ConstantMedium {
            id: IdConstructor::Single(i.id),
            density: Value::from_value(i.density)?,
            boundary: IdReference::Single(i.boundary.get_id()),
            phase_function: IdReference::Single(i.phase_function.get_id()),
        })
    }

    pub fn to_volume(
        &self,
        index: usize,
        boundary: Arc<dyn Geometry>,
        phase_function: Arc<dyn Material>,
    ) -> Result<volume::ConstantMedium, Box<dyn Error>> {
        Ok(volume::ConstantMedium::new_id(
            self.id.get_id(index),
            self.density.to_value()?,
            boundary,
            phase_function,
        ))
    }
}

#[cfg(test)]
mod constant_medium_test {
    use super::*;
    use ray_tracing_core::geometry::shape;
    use ray_tracing_core::material;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Point3;

    #[test]
    fn constant_medium_test_from_constant_medium() {
        let s = shape::Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(material::NoMaterial::new()),
        );
        let s_id = s.id;
        let m = material::NoMaterial::new();
        let m_id = m.id;
        let i = volume::ConstantMedium::new(0.5, Arc::new(s), Arc::new(m));
        let n = ConstantMedium::from_volume(&i).unwrap();
        assert_eq!(n.density, Value::Scalar(0.5));
        assert_eq!(n.boundary, IdReference::Single(s_id));
        assert_eq!(n.phase_function, IdReference::Single(m_id));
    }

    #[test]
    fn constant_medium_test_to_constant_medium() {
        let f = ConstantMedium {
            id: IdConstructor::Single(0),
            density: Value::Scalar(0.5),
            boundary: IdReference::Single(1),
            phase_function: IdReference::Single(2),
        };
        let i = f
            .to_volume(
                0,
                Arc::new(shape::Sphere::new(
                    Point3::new(0.0, 0.0, 0.0),
                    1.0,
                    Arc::new(material::NoMaterial::new()),
                )),
                Arc::new(material::NoMaterial::new()),
            )
            .unwrap();
        test::assert_eq_float(i.density, 0.5, 0.001);
    }
}
