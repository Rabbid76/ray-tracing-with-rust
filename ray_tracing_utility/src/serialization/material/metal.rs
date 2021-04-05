use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::material;
use ray_tracing_core::texture::Texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Metal {
    pub id: IdConstructor,
    pub fuzz: Value,
    pub albedo: IdReference,
}

impl Metal {
    pub fn from_material(m: &material::Metal) -> Result<Metal, Box<dyn Error>> {
        Ok(Metal {
            id: IdConstructor::Single(m.id),
            fuzz: Value::from_value(m.fuzz)?,
            albedo: IdReference::Single(m.albedo.get_id()),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        albedo: Arc<dyn Texture>,
    ) -> Result<material::Metal, Box<dyn Error>> {
        Ok(material::Metal {
            id: self.id.get_id(index),
            fuzz: self.fuzz.to_value()?,
            albedo,
        })
    }
}

#[cfg(test)]
mod metal_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGBA, Point3, TextureCoordinate};

    #[test]
    fn metal_test_form_material() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0)));
        let ct_id = ct.id;
        let m = material::Metal::new(0.5, ct);
        let ma = Metal::from_material(&m).unwrap();
        assert_eq!(ma.fuzz, Value::Scalar(0.5));
        assert_eq!(ma.albedo, IdReference::Single(ct_id));
    }

    #[test]
    fn metal_test_to_material() {
        let ma = Metal {
            id: IdConstructor::Single(0),
            fuzz: Value::Scalar(0.5),
            albedo: IdReference::Single(1),
        };
        let m = ma
            .to_material(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0))),
            )
            .unwrap();
        let c = m.albedo.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(0.0, 0.0, 0.0),
        );
        assert_eq!(m.fuzz, 0.5);
        test::assert_eq_vector4(&c, &ColorRGBA::new(0.0, 0.5, 1.0, 1.0), 0.001);
    }
}
