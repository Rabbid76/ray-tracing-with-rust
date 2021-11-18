use crate::core::object::Object;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};
use std::error::Error;
use std::sync::Arc;

pub struct BlendTexture {
    pub id: usize,
    pub scale: Vector3,
    pub first_texture: Arc<dyn Texture>,
    pub second_texture: Arc<dyn Texture>,
    pub mask_texture: Arc<dyn Texture>,
}

impl BlendTexture {
    pub fn new(
        scale: Vector3,
        first_texture: Arc<dyn Texture>,
        second_texture: Arc<dyn Texture>,
        mask_texture: Arc<dyn Texture>,
    ) -> BlendTexture {
        BlendTexture {
            id: Object::new_id(),
            scale,
            first_texture,
            second_texture,
            mask_texture,
        }
    }
}

impl Texture for BlendTexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA {
        let w = self.mask_texture.value(uv, p).w;
        glm::mix(
            self.first_texture.value(uv, p),
            self.second_texture.value(uv, p),
            ColorRGBA::new(w, w, w, w),
        )
    }

    fn has_alpha(&self) -> bool {
        self.first_texture.has_alpha() || self.second_texture.has_alpha()
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_blend_texture(&self)
    }
}

#[cfg(test)]
mod blend_texture_test {
    use super::*;
    use crate::random;
    use crate::texture::ConstantTexture;

    #[test]
    fn value_test() {
        let t1 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.5, 0.2, 1.0)));
        let t2 = Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 0.0, 0.2, 1.0)));
        let t3 = Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 0.5)));
        let t = BlendTexture::new(Vector3::new(1.0, 1.0, 1.0), t1, t2, t3);
        let c1 = t.value(
            &random::generate_uv(),
            &Vector3::new(1.5708, 1.5708, 1.5708),
        );
        assert_eq!(c1, ColorRGBA::new(0.5, 0.25, 0.2, 1.0));
        let c2 = t.value(
            &random::generate_uv(),
            &Vector3::new(-1.5708, -1.5708, -1.5708),
        );
        assert_eq!(c2, ColorRGBA::new(0.5, 0.25, 0.2, 1.0));
    }
}
