use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::material;
use ray_tracing_core::texture::Texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Lambertian {
    pub id: IdConstructor,
    pub albedo: IdReference,
}

impl Lambertian {
    pub fn from_material(m: &material::Lambertian) -> Result<Lambertian, Box<dyn Error>> {
        Ok(Lambertian {
            id: IdConstructor::Single(m.id),
            albedo: IdReference::Single(m.albedo.get_id()),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        albedo: Arc<dyn Texture>,
    ) -> Result<material::Lambertian, Box<dyn Error>> {
        Ok(material::Lambertian {
            id: self.id.get_id(index),
            albedo,
        })
    }
}

#[cfg(test)]
mod lambertian_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGBA, Point3, TextureCoordinate};

    #[test]
    fn lambertian_test_form_material() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0)));
        let ct_id = ct.id;
        let m = material::Lambertian::new(ct);
        let l = Lambertian::from_material(&m).unwrap();
        assert_eq!(l.albedo, IdReference::Single(ct_id));
    }

    #[test]
    fn lambertian_test_to_material() {
        let l = Lambertian {
            id: IdConstructor::Single(0),
            albedo: IdReference::Single(1),
        };
        let m = l
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
