use crate::core::object::Object;
use crate::texture::{Texture, Visitor};
use crate::types::{ColorRGBA, FSize, Point3, TextureCoordinate};
use std::error::Error;

pub struct BitmapTexture {
    pub id: usize,
    pub nx: usize,
    pub ny: usize,
    pub data: Vec<u8>,
    alpha_texture: bool,
}

impl BitmapTexture {
    pub fn new(nx: usize, ny: usize, data: Vec<u8>) -> BitmapTexture {
        BitmapTexture::new_id(Object::new_id(), nx, ny, data)
    }

    pub fn new_id(id: usize, nx: usize, ny: usize, data: Vec<u8>) -> BitmapTexture {
        let alpha_texture = (0..data.len() / 4).any(|i| data[i * 4 + 3] < 255);
        BitmapTexture {
            id,
            nx,
            ny,
            data,
            alpha_texture,
        }
    }
}

impl Texture for BitmapTexture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn value(&self, uv: &TextureCoordinate, _: &Point3) -> ColorRGBA {
        let u = usize::clamp((self.nx as FSize * uv.u) as usize % self.nx, 0, self.nx - 1);
        let v = usize::clamp(
            (self.ny as FSize * (1.0 - uv.v)) as usize % self.ny,
            0,
            self.ny - 1,
        );
        let i = u * 4 + v * self.nx * 4;
        ColorRGBA::new(
            self.data[i] as FSize,
            self.data[i + 1] as FSize,
            self.data[i + 2] as FSize,
            self.data[i + 3] as FSize,
        ) / 255.0
    }

    fn has_alpha(&self) -> bool {
        self.alpha_texture
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_bitmap_texture(&self)
    }
}

#[cfg(test)]
mod bitmap_texture_test {
    use super::*;
    use crate::random;
    use crate::test;

    #[test]
    fn value_test() {
        let t = BitmapTexture::new(
            2,
            2,
            vec![
                255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
            ],
        );
        let c = t.value(
            &TextureCoordinate::from_uv(0.0, 0.0),
            &random::generate_point3(),
        );
        test::assert_eq_vector4(&c, &ColorRGBA::new(1.0, 0.0, 0.0, 1.0), 0.001);
    }
}
