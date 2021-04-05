use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::random;
use crate::texture::Texture;
use crate::types;
use crate::types::{ColorRGB, ColorRGBA, FSize};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct Dielectric {
    pub id: usize,
    pub ref_idx: Range<FSize>,
    pub albedo: Arc<dyn Texture>,
}

impl Dielectric {
    pub fn new(ref_idx: Range<FSize>, albedo: Arc<dyn Texture>) -> Dielectric {
        Dielectric {
            id: Object::new_id(),
            ref_idx,
            albedo,
        }
    }

    fn hue_to_rgb(h: FSize) -> ColorRGBA {
        let r = glm::clamp(FSize::abs(h * 6.0 + 3.0) - 1.0, 0.0, 1.0);
        let g = glm::clamp(2.0 - FSize::abs(h * 6.0 - 2.0), 0.0, 1.0);
        let b = glm::clamp(2.0 - FSize::abs(h * 6.0 - 4.0), 0.0, 1.0);
        ColorRGBA::new(r, g, b, 1.0)
    }
}

impl Material for Dielectric {
    fn get_id(&self) -> usize {
        self.id
    }

    fn scatter(
        &self,
        self_material: Arc<dyn Material>,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatterRecord> {
        let r_dot_n = glm::dot(ray_in.direction, hit_record.normal);

        let mut albedo = self.albedo.value(&hit_record.uv, &hit_record.position);
        let mut w = ray_in.w;
        if self.ref_idx.end > self.ref_idx.start + 0.00001 && w.is_none() {
            let w_value = random::generate_size();
            albedo = albedo * Dielectric::hue_to_rgb((1.0 - w_value) * 300.0 / 360.0);
            w = Some(w_value);
        }
        let ref_idx = match w {
            Some(w_value) => glm::mix(self.ref_idx.start, self.ref_idx.end, w_value),
            None => self.ref_idx.start,
        };

        let (outward_normal, ni_over_nt, cosine) = if r_dot_n > 0.0 {
            (
                -hit_record.normal,
                ref_idx,
                ref_idx * r_dot_n / glm::length(ray_in.direction),
            )
        } else {
            (
                hit_record.normal,
                1.0 / ref_idx,
                -r_dot_n / glm::length(ray_in.direction),
            )
        };

        let direction = match types::refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_probe = types::schlick(cosine, ref_idx);
                if random::generate_size() < reflect_probe {
                    glm::reflect(ray_in.direction, hit_record.normal)
                } else {
                    refracted
                }
            }
            None => glm::reflect(ray_in.direction, hit_record.normal),
            //None => Vector3::new(0.0, 0.0, 0.0),
        };
        Some(ScatterRecord::new(
            Ray::new(hit_record.position, direction, ray_in.time, w),
            true,
            albedo.truncate(3),
            albedo.w,
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
        visitor.visit_dielectric(&self)
    }
}

#[cfg(test)]
mod dielectric_test {
    use super::*;
    use crate::material::NoMaterial;
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGB, Point3, TextureCoordinate, Vector3};
    use std::sync::Arc;

    #[test]
    fn scatter_test() {
        let m = Arc::new(Dielectric::new(
            0.5..0.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        ));
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
                    &ColorRGB::new(1.0, 1.0, 1.0),
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
                    Vector3::new(-2.0, -2.0, -2.0)..Vector3::new(2.0, 2.0, 2.0),
                );
                assert!(glm::length(scatter_record.ray.direction) <= 2.0);
                test::assert_eq_float(scatter_record.ray.time, 0.0, 0.001);
            }
            None => panic!("no result"),
        }
    }

    #[test]
    fn emitted_test() {
        let m = Dielectric::new(
            0.5..0.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        );
        let c = m.emitted(
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::empty(),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(0.0, 0.0, 0.0), 0.01);
    }
}
