use crate::types::{ColorRGBA, Point3, TextureCoordinate};
use std::error::Error;

mod bitmap_texture;
pub use self::bitmap_texture::BitmapTexture;

mod checker_texture;
pub use self::checker_texture::CheckerTexture;

mod constant_texture;
pub use self::constant_texture::ConstantTexture;

mod noise_texture;
pub use self::noise_texture::{NoiseTexture, NoiseType};

mod color_filter;
pub use self::color_filter::ColorFilter;

pub trait Texture: Sync + Send {
    fn get_id(&self) -> usize;

    /// Look up color in texture
    fn value(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA;

    fn has_alpha(&self) -> bool;

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor {
    fn visit_constant_texture(&mut self, t: &ConstantTexture) -> Result<(), Box<dyn Error>>;
    fn visit_bitmap_texture(&mut self, t: &BitmapTexture) -> Result<(), Box<dyn Error>>;
    fn visit_checker_texture(&mut self, t: &CheckerTexture) -> Result<(), Box<dyn Error>>;
    fn visit_noise_texture(&mut self, t: &NoiseTexture) -> Result<(), Box<dyn Error>>;
    fn visit_color_filter(&mut self, t: &ColorFilter) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod test_visitor {
    use super::*;
    use crate::types::{ColorRGBA, Vector3};
    use std::sync::Arc;

    struct TestVisitor {
        pub count: Vec<usize>,
    }

    impl TestVisitor {
        pub fn default() -> TestVisitor {
            TestVisitor {
                count: vec![0, 0, 0, 0, 0],
            }
        }

        pub fn evaluate(&self, index: usize, expected: usize) {
            for (i, count) in self.count.iter().enumerate() {
                assert_eq!(count, &if i == index { expected } else { 0 });
            }
        }
    }

    impl Visitor for TestVisitor {
        fn visit_constant_texture(&mut self, _: &ConstantTexture) -> Result<(), Box<dyn Error>> {
            self.count[0] += 1;
            Ok(())
        }
        fn visit_bitmap_texture(&mut self, _: &BitmapTexture) -> Result<(), Box<dyn Error>> {
            self.count[1] += 1;
            Ok(())
        }
        fn visit_checker_texture(&mut self, _: &CheckerTexture) -> Result<(), Box<dyn Error>> {
            self.count[2] += 1;
            Ok(())
        }
        fn visit_noise_texture(&mut self, _: &NoiseTexture) -> Result<(), Box<dyn Error>> {
            self.count[3] += 1;
            Ok(())
        }
        fn visit_color_filter(&mut self, _: &ColorFilter) -> Result<(), Box<dyn Error>> {
            self.count[4] += 1;
            Ok(())
        }
    }

    #[test]
    pub fn test_visitor_constant_texture() {
        let t = ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        let mut v = TestVisitor::default();
        t.accept(&mut v).unwrap();
        v.evaluate(0, 1);
    }

    #[test]
    pub fn test_visitor_bitmap_texture() {
        let t = BitmapTexture::new(1, 1, vec![255, 0, 0, 255]);
        let mut v = TestVisitor::default();
        t.accept(&mut v).unwrap();
        v.evaluate(1, 1);
    }

    #[test]
    pub fn test_visitor_checker_texture() {
        let e = ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        let o = ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0));
        let t = CheckerTexture::new(Vector3::new(0.0, 0.0, 0.0), Arc::new(e), Arc::new(o));
        let mut v = TestVisitor::default();
        t.accept(&mut v).unwrap();
        v.evaluate(2, 1);
    }

    #[test]
    pub fn test_visitor_noise_texture() {
        let ct1 = ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        let ct2 = ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0));
        let t = NoiseTexture::new(1.0, NoiseType::Default, Arc::new(ct1), Arc::new(ct2));
        let mut v = TestVisitor::default();
        t.accept(&mut v).unwrap();
        v.evaluate(3, 1);
    }

    #[test]
    pub fn test_visitor_color_filter() {
        let ct = ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0));
        let t = ColorFilter::new(
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            ColorRGBA::new(0.0, 0.5, 0.5, 1.0),
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            Arc::new(ct),
        );
        let mut v = TestVisitor::default();
        t.accept(&mut v).unwrap();
        v.evaluate(4, 1);
    }
}
