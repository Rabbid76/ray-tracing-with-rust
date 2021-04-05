use crate::core::ScatterRecord;
use crate::material::{Material, NoMaterial};
use crate::math::Ray;
use crate::random;
use crate::types::{FSize, Point3, TextureCoordinate, Vector3};
use std::sync::Arc;

/// Object that stores information when a ray hits an object such as shape or volume.
pub struct HitRecord {
    pub t: FSize,
    pub uv: TextureCoordinate,
    pub position: Point3,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub scatter_result: Option<ScatterRecord>,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            uv: TextureCoordinate::from_uv(0.0, 0.0),
            position: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Arc::new(NoMaterial::new()),
            scatter_result: None,
        }
    }

    pub fn new(
        t: FSize,
        uv: TextureCoordinate,
        position: Point3,
        normal: Vector3,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            t,
            uv,
            position,
            normal,
            material: material,
            scatter_result: None,
        }
    }

    pub fn new_with_color(
        t: FSize,
        uv: TextureCoordinate,
        position: Point3,
        normal: Vector3,
        material: Arc<dyn Material>,
        scatter_result: Option<ScatterRecord>,
    ) -> HitRecord {
        HitRecord {
            t,
            uv,
            position,
            normal,
            material,
            scatter_result,
        }
    }

    pub fn from_hit_record(hit_record: &HitRecord) -> HitRecord {
        HitRecord::new(
            hit_record.t,
            TextureCoordinate::from_uv(hit_record.uv.u, hit_record.uv.v),
            hit_record.position,
            hit_record.normal,
            hit_record.material.clone(),
        )
    }

    pub fn check_alpha_and_create(
        ray_in: &Ray,
        t: FSize,
        uv: TextureCoordinate,
        position: Point3,
        normal: Vector3,
        material: Arc<dyn Material>,
    ) -> Option<HitRecord> {
        if material.has_alpha() {
            let mut hit_record = HitRecord::new(t, uv, position, normal, material.clone());
            match hit_record
                .material
                .scatter(material.clone(), ray_in, &hit_record)
            {
                Some(scatter_record) => {
                    if random::generate_size() < scatter_record.alpha {
                        hit_record.scatter_result = Some(scatter_record);
                        Some(hit_record)
                    } else {
                        None
                    }
                }
                None => None,
            }
        } else {
            Some(HitRecord::new(t, uv, position, normal, material))
        }
    }

    pub fn invert_normal(&mut self) {
        self.normal = -self.normal;
    }

    pub fn displace(&mut self, offset: Vector3) {
        self.position = self.position + offset
    }

    pub fn scatter(&self, ray_in: &Ray) -> Option<ScatterRecord> {
        match &self.scatter_result {
            Some(scatter_result) => Some(scatter_result.clone()),
            None => self.material.scatter(self.material.clone(), ray_in, &self),
        }
    }
}

#[cfg(test)]
mod hit_record_test {
    use super::*;
    use crate::test;

    #[test]
    fn invert_normal_test() {
        let hr = &mut HitRecord::new(
            0.0,
            TextureCoordinate::from_uv(0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 2.0, 3.0),
            Arc::new(NoMaterial::new()),
        );
        hr.invert_normal();
        test::assert_eq_vector3(&hr.normal, &Vector3::new(-1.0, -2.0, -3.0), 0.001)
    }

    #[test]
    fn displace_test() {
        let hr = &mut HitRecord::new(
            0.0,
            TextureCoordinate::from_uv(0.0, 0.0),
            Point3::new(1.0, 2.0, 3.0),
            Vector3::new(0.0, 0.0, 0.0),
            Arc::new(NoMaterial::new()),
        );
        hr.displace(Vector3::new(0.5, 1.5, 2.5));
        test::assert_eq_vector3(&hr.position, &Vector3::new(1.5, 3.5, 5.5), 0.001)
    }
}
