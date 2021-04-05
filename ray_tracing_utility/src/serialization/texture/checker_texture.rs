use crate::serialization::Value;
use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CheckerTexture {
    pub id: IdConstructor,
    pub scale: Value,
    pub even_texture: IdReference,
    pub odd_texture: IdReference,
}

impl CheckerTexture {
    pub fn from_texture(t: &texture::CheckerTexture) -> Result<CheckerTexture, Box<dyn Error>> {
        Ok(CheckerTexture {
            id: IdConstructor::Single(t.id),
            scale: Value::from_vector3(t.scale)?,
            even_texture: IdReference::Single(t.even_texture.get_id()),
            odd_texture: IdReference::Single(t.odd_texture.get_id()),
        })
    }

    pub fn to_texture(
        &self,
        index: usize,
        even_texture: Arc<dyn texture::Texture>,
        odd_texture: Arc<dyn texture::Texture>,
    ) -> Result<texture::CheckerTexture, Box<dyn Error>> {
        Ok(texture::CheckerTexture {
            id: self.id.get_id(index),
            scale: self.scale.to_vector3()?,
            even_texture,
            odd_texture,
        })
    }
}

#[cfg(test)]
mod texture_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};

    #[test]
    fn texture_test_form_texture() {
        let ct1 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let ct1_id = ct1.id;
        let ct2 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let ct2_id = ct2.id;
        let ct = Arc::new(texture::CheckerTexture::new(
            Vector3::new(1.0, 1.0, 1.0),
            ct1,
            ct2,
        ));
        let t = CheckerTexture::from_texture(&ct).unwrap();
        assert_eq!(t.even_texture, IdReference::Single(ct1_id));
        assert_eq!(t.odd_texture, IdReference::Single(ct2_id));
    }

    #[test]
    fn texture_test_to_texture() {
        let ct = CheckerTexture {
            id: IdConstructor::Single(2),
            scale: Value::Vector3((2.0, 3.0, 4.0)),
            even_texture: IdReference::Single(0),
            odd_texture: IdReference::Single(1),
        };
        let t = ct
            .to_texture(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            )
            .unwrap();
        let c1 = t.even_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(1.5708, 1.5708, 1.5708),
        );
        test::assert_eq_vector4(&c1, &ColorRGBA::new(0.0, 0.0, 0.0, 1.0), 0.001);
        let c2 = t.odd_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(-1.5708, -1.5708, -1.5708),
        );
        test::assert_eq_vector4(&c2, &ColorRGBA::new(1.0, 1.0, 1.0, 1.0), 0.001);
        test::assert_eq_vector3(&t.scale, &Vector3::new(2.0, 3.0, 4.0), 0.001);
    }
}
