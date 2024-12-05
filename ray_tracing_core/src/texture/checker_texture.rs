use crate::core::object::Object;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, FSize, Point3, TextureCoordinate, Vector3};
use std::error::Error;
use std::sync::Arc;

pub struct CheckerTexture {
    pub id: usize,
    pub scale: Vector3,
    pub even_texture: Arc<dyn Texture>,
    pub odd_texture: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(
        scale: Vector3,
        even_texture: Arc<dyn Texture>,
        odd_texture: Arc<dyn Texture>,
    ) -> CheckerTexture {
        CheckerTexture {
            id: Object::new_id(),
            scale,
            even_texture,
            odd_texture,
        }
    }
}

impl Texture for CheckerTexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA {
        let sines = FSize::sin(self.scale.x * p.x)
            * FSize::sin(self.scale.y * p.y)
            * FSize::sin(self.scale.z * p.z);
        if sines < 0.0 {
            self.odd_texture.value(uv, p)
        } else {
            self.even_texture.value(uv, p)
        }
    }

    fn has_alpha(&self) -> bool {
        self.even_texture.has_alpha() || self.odd_texture.has_alpha()
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_checker_texture(&self)
    }
}

#[cfg(test)]
mod checker_texture_test {
    use super::*;
    use crate::random;
    use crate::texture::ConstantTexture;

    #[test]
    fn value_test() {
        let t1 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0)));
        let t2 = Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0)));
        let t = CheckerTexture::new(Vector3::new(1.0, 1.0, 1.0), t1, t2);
        let c1 = t.value(
            &random::generate_uv(),
            &Vector3::new(1.5708, 1.5708, 1.5708),
        );
        assert_eq!(c1, ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        let c2 = t.value(
            &random::generate_uv(),
            &Vector3::new(-1.5708, -1.5708, -1.5708),
        );
        assert_eq!(c2, ColorRGBA::new(1.0, 1.0, 1.0, 1.0));
    }
}
