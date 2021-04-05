use crate::core::object::Object;
use crate::core::HitRecord;
use crate::hit_able::{HitAble, Visitor};
use crate::material::Material;
use crate::math::{Ray, AABB};
use crate::random;
use crate::types::{FSize, TextureCoordinate, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct XZRect {
    pub id: usize,
    pub rect: Range<(FSize, FSize)>,
    pub k: FSize,
    pub material: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(rect: Range<(FSize, FSize)>, k: FSize, material: Arc<dyn Material>) -> XZRect {
        XZRect {
            id: Object::new_id(),
            rect,
            k,
            material,
        }
    }

    fn calculate_uv(&self, u: FSize, v: FSize) -> (FSize, FSize) {
        (
            (u - self.rect.start.0) / (self.rect.end.0 - self.rect.start.0),
            (v - self.rect.start.1) / (self.rect.end.1 - self.rect.start.1),
        )
    }

    fn area(&self) -> FSize {
        (self.rect.end.0 - self.rect.start.0) * (self.rect.end.1 - self.rect.start.1)
    }
}

impl HitAble for XZRect {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, _: Range<FSize>) -> Option<AABB> {
        Some(AABB::new(
            Vector3::new(self.rect.start.0, self.k - 0.0001, self.rect.start.1),
            Vector3::new(self.rect.end.0, self.k + 0.0001, self.rect.end.1),
        ))
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.rect.start.0
            || x > self.rect.end.0
            || z < self.rect.start.1
            || z > self.rect.end.1
        {
            return None;
        }

        let uv = self.calculate_uv(x, z);
        HitRecord::check_alpha_and_create(
            ray,
            t,
            TextureCoordinate::from_uv(uv.0, uv.1),
            ray.point_at(t),
            Vector3::new(0.0, 1.0, 0.0),
            self.material.clone(),
        )
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        match self.hit(&Ray::new_ray(*o, *v), 0.001..FSize::MAX) {
            Some(hit_record) => {
                let area = self.area();
                let distance_squared = hit_record.t * hit_record.t * glm::dot(*v, *v);
                let cosine = FSize::abs(glm::dot(*v, hit_record.normal) / glm::length(*v));
                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        let xz = random::generate_range2d(&self.rect);
        Vector3::new(xz.0, self.k, xz.1) - *o
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_shape_xz_rect(&self)
    }
}

#[cfg(test)]
mod xz_rect_test {
    use super::*;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;

    #[test]
    fn bounding_box_test() {
        let r = XZRect::new((0.0, 0.0)..(1.0, 1.0), 0.0, Arc::new(NoMaterial::new()));
        let b = r.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(0.0, 0.0, 0.0), 0.01);
                test::assert_eq_vector3(&b.max, &Vector3::new(1.0, 0.0, 1.0), 0.01);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn hit_test() {
        let r = XZRect::new(
            (0.0, 0.0)..(1.0, 1.0),
            0.5,
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            )))),
        );
        let ray1 = Ray::new_ray(Vector3::new(-1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray3 = Ray::new_ray(Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
        match r.hit(&ray1, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match r.hit(&ray2, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match r.hit(&ray3, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
