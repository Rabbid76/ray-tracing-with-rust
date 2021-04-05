use crate::core::object::Object;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, Point3, TextureCoordinate};
use std::error::Error;

/// Texture object that represents a uniform color
pub struct ConstantTexture {
    pub id: usize,
    pub color: ColorRGBA,
}

impl ConstantTexture {
    pub fn new(color: ColorRGBA) -> ConstantTexture {
        ConstantTexture {
            id: Object::new_id(),
            color,
        }
    }
}

impl Texture for ConstantTexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, _: &TextureCoordinate, _: &Point3) -> ColorRGBA {
        self.color
    }

    fn has_alpha(&self) -> bool {
        self.color.w < 1.0
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_constant_texture(&self)
    }
}

#[cfg(test)]
mod constant_texture_test {
    use super::*;
    use crate::random;

    #[test]
    fn value_test() {
        let t = ConstantTexture::new(ColorRGBA::new(1.0, 0.0, 0.0, 1.0));
        let c = t.value(&random::generate_uv(), &random::generate_point3());
        assert_eq!(c, ColorRGBA::new(1.0, 0.0, 0.0, 1.0));
    }
}
