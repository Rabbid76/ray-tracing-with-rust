use crate::serialization::Value;
use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlendTexture {
    pub id: IdConstructor,
    pub scale: Value,
    pub first_texture: IdReference,
    pub second_texture: IdReference,
    pub mask_texture: IdReference,
}

impl BlendTexture {
    pub fn from_texture(t: &texture::BlendTexture) -> Result<BlendTexture, Box<dyn Error>> {
        Ok(BlendTexture {
            id: IdConstructor::Single(t.id),
            scale: Value::from_vector3(t.scale)?,
            first_texture: IdReference::Single(t.first_texture.get_id()),
            second_texture: IdReference::Single(t.second_texture.get_id()),
            mask_texture: IdReference::Single(t.mask_texture.get_id()),
        })
    }

    pub fn to_texture(
        &self,
        index: usize,
        first_texture: Arc<dyn texture::Texture>,
        second_texture: Arc<dyn texture::Texture>,
        mask_texture: Arc<dyn texture::Texture>,
    ) -> Result<texture::BlendTexture, Box<dyn Error>> {
        Ok(texture::BlendTexture {
            id: self.id.get_id(index),
            scale: self.scale.to_vector3()?,
            first_texture,
            second_texture,
            mask_texture,
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
        let ct3 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let ct3_id = ct3.id;
        let ct = Arc::new(texture::BlendTexture::new(
            Vector3::new(1.0, 1.0, 1.0),
            ct1,
            ct2,
            ct3,
        ));
        let t = BlendTexture::from_texture(&ct).unwrap();
        assert_eq!(t.first_texture, IdReference::Single(ct1_id));
        assert_eq!(t.second_texture, IdReference::Single(ct2_id));
        assert_eq!(t.mask_texture, IdReference::Single(ct3_id));
    }

    #[test]
    fn texture_test_to_texture() {
        let ct = BlendTexture {
            id: IdConstructor::Single(2),
            scale: Value::Vector3((2.0, 3.0, 4.0)),
            first_texture: IdReference::Single(0),
            second_texture: IdReference::Single(1),
            mask_texture: IdReference::Single(1),
        };
        let t = ct
            .to_texture(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.5, 0.5, 0.5, 0.5))),
            )
            .unwrap();
        let c1 = t.first_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(1.5708, 1.5708, 1.5708),
        );
        test::assert_eq_vector4(&c1, &ColorRGBA::new(0.0, 0.0, 0.0, 1.0), 0.001);
        let c2 = t.second_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(-1.5708, -1.5708, -1.5708),
        );
        test::assert_eq_vector4(&c2, &ColorRGBA::new(1.0, 1.0, 1.0, 1.0), 0.001);
        let c3 = t.mask_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(-1.5708, -1.5708, -1.5708),
        );
        test::assert_eq_vector4(&c3, &ColorRGBA::new(0.5, 0.5, 0.5, 0.5), 0.001);
        test::assert_eq_vector3(&t.scale, &Vector3::new(2.0, 3.0, 4.0), 0.001);
    }
}
