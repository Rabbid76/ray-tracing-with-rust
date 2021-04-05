use crate::core::object::Object;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, Point3, TextureCoordinate};
use std::error::Error;
use std::sync::Arc;

/// Color filter for textures
///
/// ```lang--none
/// DST_rgba = clamp(SRC_rgba * SRC_rgba * A_rgba + SRC_rgba * B_rgba + C_rgba, 0.0, 1.0)
/// ```
pub struct ColorFilter {
    pub id: usize,
    pub a: ColorRGBA,
    pub b: ColorRGBA,
    pub c: ColorRGBA,
    pub texture: Arc<dyn Texture>,
}

impl ColorFilter {
    pub fn new(a: ColorRGBA, b: ColorRGBA, c: ColorRGBA, texture: Arc<dyn Texture>) -> ColorFilter {
        ColorFilter {
            id: Object::new_id(),
            a,
            b,
            c,
            texture,
        }
    }
}

impl Texture for ColorFilter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA {
        let color = self.texture.value(uv, p);
        color * color * self.a + color * self.b + self.c
    }

    fn has_alpha(&self) -> bool {
        self.texture.has_alpha() || self.a.w + self.b.w + self.c.w < 0.9999
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_color_filter(&self)
    }
}

#[cfg(test)]
mod checker_texture_test {
    use super::*;
    use crate::random;
    use crate::texture::ConstantTexture;

    #[test]
    fn value_test() {
        let t = Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0)));
        let cf = ColorFilter::new(
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            ColorRGBA::new(1.0, 0.5, 0.25, 1.0),
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            t,
        );
        let c = cf.value(&random::generate_uv(), &random::generate_vector3());
        assert_eq!(c, ColorRGBA::new(1.0, 0.5, 0.25, 1.0));
    }
}
