use crate::core::object::Object;
use crate::math::Perlin;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, FSize, Point3, TextureCoordinate};
use std::error::Error;
use std::sync::Arc;

pub enum NoiseType {
    Default = 0,
    Turb = 1,
    SinX = 2,
    SinY = 3,
    SinZ = 4,
}

pub struct NoiseTexture {
    pub id: usize,
    pub scale: FSize,
    pub noise_type: NoiseType,
    pub min_texture: Arc<dyn Texture>,
    pub max_texture: Arc<dyn Texture>,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(
        scale: FSize,
        noise_type: NoiseType,
        min_texture: Arc<dyn Texture>,
        max_texture: Arc<dyn Texture>,
    ) -> NoiseTexture {
        NoiseTexture::new_id(
            Object::new_id(),
            scale,
            noise_type,
            min_texture,
            max_texture,
        )
    }

    pub fn new_id(
        id: usize,
        scale: FSize,
        noise_type: NoiseType,
        min_texture: Arc<dyn Texture>,
        max_texture: Arc<dyn Texture>,
    ) -> NoiseTexture {
        NoiseTexture {
            id,
            scale,
            noise_type,
            min_texture,
            max_texture,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA {
        let noise = match self.noise_type {
            NoiseType::Default => self.noise.noise(&(*p * self.scale)),
            NoiseType::Turb => self.noise.turb(*p * self.scale, 7),
            NoiseType::SinX => {
                FSize::sin(self.scale * p.x + 10.0 * self.noise.turb(*p * self.scale, 7))
            }
            NoiseType::SinY => {
                FSize::sin(self.scale * p.y + 10.0 * self.noise.turb(*p * self.scale, 7))
            }
            NoiseType::SinZ => {
                FSize::sin(self.scale * p.z + 10.0 * self.noise.turb(*p * self.scale, 7))
            }
        };
        let w = noise * 0.5 + 0.5;
        self.min_texture.value(uv, p) * (1.0 - w) + self.max_texture.value(uv, p) * w
    }

    fn has_alpha(&self) -> bool {
        self.min_texture.has_alpha() || self.max_texture.has_alpha()
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_noise_texture(&self)
    }
}

#[cfg(test)]
mod noise_texture_test {
    use super::*;
    use crate::random;
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGB, Vector3};

    #[test]
    fn value_test() {
        let ct1 = ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        let ct2 = ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0));
        let t = NoiseTexture::new(1.0, NoiseType::Default, Arc::new(ct1), Arc::new(ct2));
        let c = t.value(&random::generate_uv(), &Vector3::new(0.0, 0.0, 0.0));
        test::assert_in_range_vector3(
            c.truncate(3),
            ColorRGB::new(0.0, 0.0, 0.0)..ColorRGB::new(1.0, 1.0, 1.0),
        );
    }
}
