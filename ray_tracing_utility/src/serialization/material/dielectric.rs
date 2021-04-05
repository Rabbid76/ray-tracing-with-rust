use crate::serialization::{IdConstructor, IdReference, Value};
use ray_tracing_core::material;
use ray_tracing_core::texture::Texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Dielectric {
    pub id: IdConstructor,
    pub ref_idx: Value,
    pub albedo: IdReference,
}

impl Dielectric {
    pub fn from_material(m: &material::Dielectric) -> Result<Dielectric, Box<dyn Error>> {
        Ok(Dielectric {
            id: IdConstructor::Single(m.id),
            ref_idx: Value::from_range(m.ref_idx.start..m.ref_idx.end)?,
            albedo: IdReference::Single(m.albedo.get_id()),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        albedo: Arc<dyn Texture>,
    ) -> Result<material::Dielectric, Box<dyn Error>> {
        Ok(material::Dielectric {
            id: self.id.get_id(index),
            ref_idx: self.ref_idx.to_range()?,
            albedo,
        })
    }
}

#[cfg(test)]
mod dielectric_test {
    use super::*;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::ColorRGBA;

    #[test]
    fn dielectric_test_form_material() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0)));
        let m = material::Dielectric::new(0.5..0.5, ct);
        let d = Dielectric::from_material(&m).unwrap();
        assert_eq!(d.ref_idx, Value::Range((0.5, 0.5)));
    }

    #[test]
    fn dielectric_test_to_material() {
        let d = Dielectric {
            id: IdConstructor::Single(0),
            ref_idx: Value::Range((0.5, 0.5)),
            albedo: IdReference::Single(1),
        };
        let m = d
            .to_material(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            )
            .unwrap();
        assert_eq!(m.ref_idx, 0.5..0.5);
    }
}
