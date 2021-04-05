use crate::serialization::Value;
use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NoiseTexture {
    pub id: IdConstructor,
    pub scale: Value,
    pub noise_type: String,
    pub min_texture: IdReference,
    pub max_texture: IdReference,
}

impl NoiseTexture {
    pub fn from_texture(t: &texture::NoiseTexture) -> Result<NoiseTexture, Box<dyn Error>> {
        Ok(NoiseTexture {
            id: IdConstructor::Single(t.id),
            scale: Value::from_value(t.scale)?,
            noise_type: NoiseTexture::noise_type_to_string(&t.noise_type),
            min_texture: IdReference::Single(t.min_texture.get_id()),
            max_texture: IdReference::Single(t.max_texture.get_id()),
        })
    }

    pub fn to_texture(
        &self,
        index: usize,
        min_texture: Arc<dyn texture::Texture>,
        max_texture: Arc<dyn texture::Texture>,
    ) -> Result<texture::NoiseTexture, Box<dyn Error>> {
        Ok(texture::NoiseTexture::new_id(
            self.id.get_id(index),
            self.scale.to_value()?,
            NoiseTexture::string_tp_noise_type(&self.noise_type),
            min_texture,
            max_texture,
        ))
    }

    fn noise_type_to_string(noise_type: &texture::NoiseType) -> String {
        match noise_type {
            texture::NoiseType::Default => String::from("default"),
            texture::NoiseType::Turb => String::from("turb"),
            texture::NoiseType::SinX => String::from("sin x"),
            texture::NoiseType::SinY => String::from("sin y"),
            texture::NoiseType::SinZ => String::from("sin z"),
        }
    }

    fn string_tp_noise_type(name: &str) -> texture::NoiseType {
        match name {
            "default" => texture::NoiseType::Default,
            "turb" => texture::NoiseType::Turb,
            "sin x" => texture::NoiseType::SinX,
            "sin y" => texture::NoiseType::SinY,
            "sin z" => texture::NoiseType::SinZ,
            _ => texture::NoiseType::Default,
        }
    }
}

#[cfg(test)]
mod noise_texture_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::{ColorRGB, ColorRGBA, Point3, TextureCoordinate};

    #[test]
    fn noise_texture_form_texture() {
        let ct1 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let ct1_id = ct1.id;
        let ct2 = Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0)));
        let ct2_id = ct2.id;
        let t = texture::NoiseTexture::new(1.0, texture::NoiseType::Default, ct1, ct2);
        let nt = NoiseTexture::from_texture(&t).unwrap();
        assert_eq!(nt.scale, Value::Scalar(1.0));
        assert_eq!(nt.noise_type, "default");
        assert_eq!(nt.min_texture, IdReference::Single(ct1_id));
        assert_eq!(nt.max_texture, IdReference::Single(ct2_id));
    }

    #[test]
    fn noise_texture_to_texture() {
        let nt = NoiseTexture {
            id: IdConstructor::Single(2),
            scale: Value::Scalar(1.0),
            noise_type: String::from("default"),
            min_texture: IdReference::Single(0),
            max_texture: IdReference::Single(1),
        };

        let t = nt
            .to_texture(
                0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            )
            .unwrap();
        assert_eq!(t.scale, 1.0);
        let c = t.min_texture.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &Point3::new(0.0, 0.0, 0.0),
        );
        test::assert_in_range_vector3(
            c.truncate(3),
            ColorRGB::new(0.0, 0.0, 0.0)..ColorRGB::new(1.0, 1.0, 1.0),
        );
    }
}
