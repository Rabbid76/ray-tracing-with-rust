use crate::core::ScatterRecord;
use crate::material::{Material, NoMaterial};
use crate::math::Ray;
use crate::random;
use crate::types::{ColorRGBA, FSize, Point3, TextureCoordinate, Vector3};
use std::sync::Arc;

/// Object that stores information when a ray hits an object such as shape or volume.
pub struct HitRecord {
    pub t: FSize,
    pub uv: TextureCoordinate,
    pub position: Point3,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub color_channels: ColorRGBA,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            uv: TextureCoordinate::from_uv(0.0, 0.0),
            position: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Arc::new(NoMaterial::new()),
            color_channels: ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn new(
        t: FSize,
        uv: TextureCoordinate,
        position: Point3,
        normal: Vector3,
        material: Arc<dyn Material>,
        color_channels: ColorRGBA,
    ) -> HitRecord {
        HitRecord {
            t,
            uv,
            position,
            normal,
            material,
            color_channels,
        }
    }

    pub fn from_hit_record(hit_record: &HitRecord) -> HitRecord {
        HitRecord::new(
            hit_record.t,
            TextureCoordinate::from_uv(hit_record.uv.u, hit_record.uv.v),
            hit_record.position,
            hit_record.normal,
            hit_record.material.clone(),
            hit_record.color_channels,
        )
    }

    pub fn check_alpha_and_create(
        _ray_in: &Ray,
        t: FSize,
        uv: TextureCoordinate,
        position: Point3,
        normal: Vector3,
        material: Arc<dyn Material>,
    ) -> Option<HitRecord> {
        let selected_material = match material.material() {
            Some(m) => m,
            None => material,
        };
        let color_channels = selected_material.color_channels(&uv, &position);
        if selected_material.has_alpha() && random::generate_size() < color_channels.w {
            None
        } else {
            Some(HitRecord::new(
                t,
                uv,
                position,
                normal,
                selected_material,
                color_channels,
            ))
        }
    }

    pub fn invert_normal(&mut self) {
        self.normal = -self.normal;
    }

    pub fn displace(&mut self, offset: Vector3) {
        self.position = self.position + offset
    }

    pub fn scatter(&self, ray_in: &Ray) -> Option<ScatterRecord> {
        self.material.scatter(self.material.clone(), ray_in, &self)
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
            ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
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
            ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
        );
        hr.displace(Vector3::new(0.5, 1.5, 2.5));
        test::assert_eq_vector3(&hr.position, &Vector3::new(1.5, 3.5, 5.5), 0.001)
    }
}
