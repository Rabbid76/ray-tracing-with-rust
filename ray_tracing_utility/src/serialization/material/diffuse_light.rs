use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::material;
use ray_tracing_core::texture::Texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DiffuseLight {
    pub id: IdConstructor,
    pub emit: IdReference,
}

impl DiffuseLight {
    pub fn from_material(m: &material::DiffuseLight) -> Result<DiffuseLight, Box<dyn Error>> {
        Ok(DiffuseLight {
            id: IdConstructor::Single(m.id),
            emit: IdReference::Single(m.emit.get_id()),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        emit: Arc<dyn Texture>,
    ) -> Result<material::DiffuseLight, Box<dyn Error>> {
        Ok(material::DiffuseLight {
            id: self.id.get_id(index),
            emit,
        })
    }
}

#[cfg(test)]
mod diffuse_light_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGBA, Point3, TextureCoordinate};

    #[test]
    fn lambertian_test_form_material() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0)));
        let ct_id = ct.id;
        let m = material::DiffuseLight::new(ct);
        let d = DiffuseLight::from_material(&m).unwrap();
        assert_eq!(d.emit, IdReference::Single(ct_id));
    }

    #[test]
    fn diffuse_light_test_to_material() {
        let d = DiffuseLight {
            id: IdConstructor::Single(0),
            emit: IdReference::Single(1),
        };
        let m = d
            .to_material(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 1.0, 1.0))),
            )
            .unwrap();
        let c = m.emit.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(0.0, 0.0, 0.0),
        );
        test::assert_eq_vector4(&c, &ColorRGBA::new(0.0, 0.5, 1.0, 1.0), 0.001);
    }
}
