use crate::serialization::IdConstructor;
use crate::serialization::Value;
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConstantTexture {
    pub id: IdConstructor,
    pub color: Value,
}

impl ConstantTexture {
    pub fn from_texture(t: &texture::ConstantTexture) -> Result<ConstantTexture, Box<dyn Error>> {
        Ok(ConstantTexture {
            id: IdConstructor::Single(t.id),
            color: Value::from_color_rgba(t.color)?,
        })
    }

    pub fn to_texture(&self, index: usize) -> Result<texture::ConstantTexture, Box<dyn Error>> {
        Ok(texture::ConstantTexture {
            id: self.id.get_id(index),
            color: self.color.to_color_rgba()?,
        })
    }
}

#[cfg(test)]
mod constant_texture_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::types::ColorRGBA;

    #[test]
    fn constant_texture_form_texture() {
        let t = texture::ConstantTexture::new(ColorRGBA::new(1.0, 0.5, 0.0, 1.0));
        let ct = ConstantTexture::from_texture(&t).unwrap();
        assert_eq!(ct.color, Value::Vector4((1.0, 0.5, 0.0, 1.0)));
    }

    #[test]
    fn constant_texture_to_texture() {
        let ct = ConstantTexture {
            id: IdConstructor::Single(0),
            color: Value::Vector4((1.0, 0.5, 0.0, 1.0)),
        };
        let t = ct.to_texture(0).unwrap();
        test::assert_eq_vector4(&t.color, &ColorRGBA::new(1.0, 0.5, 0.0, 1.0), 0.001);
    }
}
