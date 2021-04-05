use crate::serialization::Value;
use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ColorFilter {
    pub id: IdConstructor,
    pub a: Value,
    pub b: Value,
    pub c: Value,
    pub texture: IdReference,
}

impl ColorFilter {
    pub fn from_texture(f: &texture::ColorFilter) -> Result<ColorFilter, Box<dyn Error>> {
        Ok(ColorFilter {
            id: IdConstructor::Single(f.id),
            a: Value::from_color_rgba(f.a)?,
            b: Value::from_color_rgba(f.b)?,
            c: Value::from_color_rgba(f.c)?,
            texture: IdReference::Single(f.texture.get_id()),
        })
    }

    pub fn to_texture(
        &self,
        index: usize,
        texture: Arc<dyn texture::Texture>,
    ) -> Result<texture::ColorFilter, Box<dyn Error>> {
        Ok(texture::ColorFilter {
            id: self.id.get_id(index),
            a: self.a.to_color_rgba()?,
            b: self.b.to_color_rgba()?,
            c: self.c.to_color_rgba()?,
            texture,
        })
    }
}

#[cfg(test)]
mod filter_test {
    use super::*;
    use ray_tracing_core::random;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::ColorRGBA;

    #[test]
    fn color_filter_test_form_texture() {
        let ct = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let ct_id = ct.id;
        let cf = Arc::new(texture::ColorFilter::new(
            ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            ColorRGBA::new(2.0, 2.0, 2.0, 2.0),
            ColorRGBA::new(3.0, 3.0, 3.0, 3.0),
            ct,
        ));
        let f = ColorFilter::from_texture(&cf).unwrap();
        assert_eq!(f.a, Value::Vector4((1.0, 1.0, 1.0, 1.0)));
        assert_eq!(f.b, Value::Vector4((2.0, 2.0, 2.0, 2.0)));
        assert_eq!(f.c, Value::Vector4((3.0, 3.0, 3.0, 3.0)));
        assert_eq!(f.texture, IdReference::Single(ct_id));
    }

    #[test]
    fn color_filter_test_to_texture() {
        let cf = ColorFilter {
            id: IdConstructor::Single(1),
            a: Value::Vector4((1.0, 1.0, 1.0, 1.0)),
            b: Value::Vector4((2.0, 2.0, 2.0, 2.0)),
            c: Value::Vector4((3.0, 3.0, 3.0, 3.0)),
            texture: IdReference::Single(0),
        };
        let f = cf
            .to_texture(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
            )
            .unwrap();
        let c = f
            .texture
            .value(&random::generate_uv(), &random::generate_vector3());
        test::assert_eq_vector4(&c, &ColorRGBA::new(0.0, 0.0, 0.0, 1.0), 0.001);
    }
}
