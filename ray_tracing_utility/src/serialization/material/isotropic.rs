use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::material;
use ray_tracing_core::texture::Texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Isotropic {
    pub id: IdConstructor,
    pub albedo: IdReference,
}

impl Isotropic {
    pub fn from_material(m: &material::Isotropic) -> Result<Isotropic, Box<dyn Error>> {
        Ok(Isotropic {
            id: IdConstructor::Single(m.id),
            albedo: IdReference::Single(m.albedo.get_id()),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        albedo: Arc<dyn Texture>,
    ) -> Result<material::Isotropic, Box<dyn Error>> {
        Ok(material::Isotropic {
            id: self.id.get_id(index),
            albedo,
        })
    }
}

#[cfg(test)]
mod isotropic_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGBA, Point3, TextureCoordinate};

    #[test]
    fn isotropic_test_form_material() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0)));
        let ct_id = ct.id;
        let m = material::Isotropic::new(ct);
        let i = Isotropic::from_material(&m).unwrap();
        assert_eq!(i.albedo, IdReference::Single(ct_id));
    }

    #[test]
    fn isotropic_test_to_material() {
        let i = Isotropic {
            id: IdConstructor::Single(0),
            albedo: IdReference::Single(1),
        };
        let m = i
            .to_material(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0))),
            )
            .unwrap();
        let c = m.albedo.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(0.0, 0.0, 0.0),
        );
        test::assert_eq_vector4(&c, &ColorRGBA::new(0.0, 0.5, 1.0, 1.0), 0.001);
    }
}
