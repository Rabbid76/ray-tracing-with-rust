use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::math::Ray;
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::sync::Arc;

mod material_blend;
pub use self::material_blend::MaterialBlend;

mod dielectric;
pub use self::dielectric::Dielectric;

mod diffuse_light;
pub use self::diffuse_light::DiffuseLight;

mod isotropic;
pub use self::isotropic::Isotropic;

mod metal;
pub use self::metal::Metal;

mod no_material;
pub use self::no_material::NoMaterial;

mod lambertian;
pub use self::lambertian::Lambertian;

pub trait Material: Sync + Send {
    fn get_id(&self) -> usize;

    /// Scatter a ray at a point on the material.
    /// Returns a tuple with the color attenuation and outgoing ray and
    fn scatter(
        &self,
        self_material: Arc<dyn Material>,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatterRecord>;

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> FSize;

    fn has_alpha(&self) -> bool;

    /// Get emitted material color
    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord) -> ColorRGB;

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor {
    fn visit_no_material(&mut self, m: &NoMaterial) -> Result<(), Box<dyn Error>>;
    fn visit_lambertian(&mut self, m: &Lambertian) -> Result<(), Box<dyn Error>>;
    fn visit_metal(&mut self, m: &Metal) -> Result<(), Box<dyn Error>>;
    fn visit_dielectric(&mut self, m: &Dielectric) -> Result<(), Box<dyn Error>>;
    fn visit_isotropic(&mut self, m: &Isotropic) -> Result<(), Box<dyn Error>>;
    fn visit_diffuse_light(&mut self, m: &DiffuseLight) -> Result<(), Box<dyn Error>>;
    fn visit_material_blend(&mut self, m: &MaterialBlend) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod test_visitor {
    use super::*;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use std::sync::Arc;

    struct TestVisitor {
        pub count: Vec<usize>,
    }

    impl TestVisitor {
        fn default() -> TestVisitor {
            TestVisitor {
                count: vec![0, 0, 0, 0, 0, 0, 0],
            }
        }

        pub fn evaluate(&self, index: usize, expected: usize) {
            for (i, count) in self.count.iter().enumerate() {
                assert_eq!(count, &if i == index { expected } else { 0 });
            }
        }
    }

    impl Visitor for TestVisitor {
        fn visit_no_material(&mut self, _: &NoMaterial) -> Result<(), Box<dyn Error>> {
            self.count[0] += 1;
            Ok(())
        }
        fn visit_lambertian(&mut self, _: &Lambertian) -> Result<(), Box<dyn Error>> {
            self.count[1] += 1;
            Ok(())
        }
        fn visit_metal(&mut self, _: &Metal) -> Result<(), Box<dyn Error>> {
            self.count[2] += 1;
            Ok(())
        }
        fn visit_dielectric(&mut self, _: &Dielectric) -> Result<(), Box<dyn Error>> {
            self.count[3] += 1;
            Ok(())
        }
        fn visit_isotropic(&mut self, _: &Isotropic) -> Result<(), Box<dyn Error>> {
            self.count[4] += 1;
            Ok(())
        }
        fn visit_diffuse_light(&mut self, _: &DiffuseLight) -> Result<(), Box<dyn Error>> {
            self.count[5] += 1;
            Ok(())
        }
        fn visit_material_blend(&mut self, _: &MaterialBlend) -> Result<(), Box<dyn Error>> {
            self.count[6] += 1;
            Ok(())
        }
    }

    #[test]
    pub fn test_visitor_no_material() {
        let m = NoMaterial::new();
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(0, 1);
    }

    #[test]
    pub fn test_visitor_lambertian() {
        let m = Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(1, 1);
    }

    #[test]
    pub fn test_visitor_metal() {
        let m = Metal::new(
            0.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
        );
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(2, 1);
    }

    #[test]
    pub fn test_visitor_dielectric() {
        let m = Dielectric::new(
            0.5..0.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        );
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(3, 1);
    }

    #[test]
    pub fn test_visitor_isotropic() {
        let m = Isotropic::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(4, 1);
    }

    #[test]
    pub fn test_visitor_diffuse_light() {
        let m = DiffuseLight::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(5, 1);
    }

    #[test]
    pub fn test_visitor_material_blend() {
        let m = MaterialBlend::new(vec![
            (1.0, Arc::new(NoMaterial::new())),
            (1.0, Arc::new(NoMaterial::new())),
        ]);
        let mut v = TestVisitor::default();
        m.accept(&mut v).unwrap();
        v.evaluate(6, 1);
    }
}
