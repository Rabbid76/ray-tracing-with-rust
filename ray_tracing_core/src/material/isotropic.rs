use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::random;
use crate::texture::Texture;
use crate::types::{ColorRGB, ColorRGBA, FSize, Point3, TextureCoordinate};
use std::error::Error;
use std::sync::Arc;

pub struct Isotropic {
    pub id: usize,
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Isotropic {
        Isotropic {
            id: Object::new_id(),
            albedo,
        }
    }
}

impl Material for Isotropic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn color_channels(&self, uv: &TextureCoordinate, p: &Point3) -> ColorRGBA {
        self.albedo.value(uv, p)
    }

    fn scatter(
        &self,
        self_material: Arc<dyn Material>,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            Ray::new_ray_with_attributes(
                hit_record.position,
                random::generate_unit_sphere(),
                ray_in,
            ),
            true,
            hit_record.color_channels.truncate(3),
            hit_record.color_channels.w,
            None,
            self_material,
        ))
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> FSize {
        1.0
    }

    fn has_alpha(&self) -> bool {
        self.albedo.has_alpha()
    }

    fn emitted(&self, _: &Ray, _: &HitRecord) -> ColorRGB {
        ColorRGB::new(0.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_isotropic(&self)
    }
}

#[cfg(test)]
mod isotropic_test {
    use super::*;
    use crate::material::NoMaterial;
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};

    #[test]
    fn scatter_test() {
        let m = Arc::new(Isotropic::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(1.0, 0.0, 0.0, 1.0),
        ))));
        let result = m.scatter(
            m.clone(),
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::new(
                0.0,
                TextureCoordinate::from_uv(0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Arc::new(NoMaterial::new()),
                ColorRGBA::new(1.0, 0.0, 0.0, 1.0),
            ),
        );
        match result {
            Some(scatter_record) => {
                test::assert_eq_vector3(
                    &scatter_record.attenuation,
                    &ColorRGB::new(1.0, 0.0, 0.0),
                    0.001,
                );
                test::assert_eq_float(scatter_record.alpha, 1.0, 0.001);
                test::assert_eq_vector3(
                    &scatter_record.ray.origin,
                    &Point3::new(0.0, 0.0, 0.0),
                    0.001,
                );
                test::assert_in_range_vector3(
                    scatter_record.ray.direction,
                    Vector3::new(-1.0, -1.0, -1.0)..Vector3::new(1.0, 1.0, 1.0),
                );
                assert!(glm::length(scatter_record.ray.direction) <= 1.0);
                test::assert_eq_float(scatter_record.ray.time, 0.0, 0.001);
            }
            None => panic!("no result"),
        }
    }

    #[test]
    fn emitted_test() {
        let m = Isotropic::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            1.0, 0.0, 0.0, 1.0,
        ))));
        let c = m.emitted(
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::empty(),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(0.0, 0.0, 0.0), 0.01);
    }
}
