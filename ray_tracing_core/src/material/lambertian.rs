use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::{OrthoNormalBase, Ray};
use crate::probability_density_function::CosinePdf;
use crate::random;
use crate::texture::Texture;
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::f64::consts::PI;
use std::sync::Arc;

/// Material object that represents a [Lambertian](https://en.wikipedia.org/wiki/Lambertian_reflectance) material
pub struct Lambertian {
    pub id: usize,
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian {
            id: Object::new_id(),
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn get_id(&self) -> usize {
        self.id
    }

    fn scatter(
        &self,
        self_material: Arc<dyn Material>,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatterRecord> {
        let uvw = OrthoNormalBase::form_w(&hit_record.normal);
        let direction = glm::normalize(uvw.local(random::generate_cosine_direction()));
        let albedo = self.albedo.value(&hit_record.uv, &hit_record.position);
        Some(ScatterRecord::new(
            Ray::new_ray_with_attributes(hit_record.position, direction, ray_in),
            false,
            albedo.truncate(3),
            albedo.w,
            Some(Arc::new(CosinePdf::from_w(&hit_record.normal))),
            self_material,
        ))
    }

    fn scattering_pdf(&self, _: &Ray, hit_record: &HitRecord, scattered: &Ray) -> FSize {
        let n_dot_d = glm::dot(hit_record.normal, glm::normalize(scattered.direction));
        if n_dot_d < 0.0 {
            0.0
        } else {
            n_dot_d / PI
        }
    }

    fn has_alpha(&self) -> bool {
        self.albedo.has_alpha()
    }

    fn emitted(&self, _: &Ray, _: &HitRecord) -> ColorRGB {
        ColorRGB::new(0.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_lambertian(&self)
    }
}

#[cfg(test)]
mod lambertian_test {
    use super::*;
    use crate::material::NoMaterial;
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};

    #[test]
    fn scatter_test() {
        let m = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
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
            ),
        );
        match result {
            Some(scatter_record) => {
                test::assert_eq_vector3(
                    &scatter_record.attenuation,
                    &ColorRGB::new(0.0, 0.0, 0.0),
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
                    Vector3::new(-1.0, -1.0, 0.0)..Vector3::new(1.0, 1.0, 2.0),
                );
                assert!(glm::length(scatter_record.ray.direction) <= 2.0);
                test::assert_eq_float(scatter_record.ray.time, 0.0, 0.001);
            }
            None => panic!("no result"),
        }
    }

    #[test]
    fn emitted_test() {
        let m = Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let c = m.emitted(
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::empty(),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(0.0, 0.0, 0.0), 0.01);
    }
}
