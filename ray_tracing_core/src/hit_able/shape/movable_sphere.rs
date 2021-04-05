use crate::core::object::Object;
use crate::core::HitRecord;
use crate::hit_able::{HitAble, Visitor};
use crate::material::Material;
use crate::math::OrthoNormalBase;
use crate::math::{Ray, AABB};
use crate::random;
use crate::types::{FSize, Point3, TextureCoordinate, Vector3};
use core::f64::consts::PI;
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

/// Spherical shape
///
/// Object that represents a sphere in the scene. Stores the equation of a sphere.
///
/// Ray - Sphere intersection
///
/// Sphere:         dot(p-C, p-C) = R*R            `C`: center, `p`: point on the sphere, `R`, radius
/// Ray:            p(t) = A + B * t               `A`: origin, `B`: direction        
/// Intersection:   dot(A +B*t-C, A+B*t-C) = R*R
/// t*t*dot(B,B) + 2*t*dot(B,A-C) + dot(A-C,A-C) - R*R = 0
pub struct MovableSphere {
    pub id: usize,
    pub center_range: Range<Point3>,
    pub time_range: Range<FSize>,
    pub radius: FSize,
    pub material: Arc<dyn Material>,
}

impl MovableSphere {
    pub fn new(
        center_range: Range<Point3>,
        time_range: Range<FSize>,
        radius: FSize,
        material: Arc<dyn Material>,
    ) -> MovableSphere {
        MovableSphere {
            id: Object::new_id(),
            center_range,
            time_range,
            radius,
            material,
        }
    }

    fn center(&self, time: FSize) -> Point3 {
        let t = glm::clamp(
            (time - self.time_range.start) / (self.time_range.end - self.time_range.start),
            0.0,
            1.0,
        );
        self.center_range.start * (1.0 - t) + self.center_range.end * t
    }
}

impl HitAble for MovableSphere {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, time: Range<FSize>) -> Option<AABB> {
        Some(
            AABB::new(
                self.center(time.start) - Vector3::new(self.radius, self.radius, self.radius),
                self.center(time.start) + Vector3::new(self.radius, self.radius, self.radius),
            ) | AABB::new(
                self.center(time.end) - Vector3::new(self.radius, self.radius, self.radius),
                self.center(time.end) + Vector3::new(self.radius, self.radius, self.radius),
            ),
        )
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = glm::dot(ray.direction, ray.direction);
        let b = 2.0 * glm::dot(oc, ray.direction);
        let c = glm::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - FSize::sqrt(discriminant)) / (2.0 * a);
            let mut result: Option<HitRecord> = None;
            if t_range.contains(&temp) {
                let p = ray.point_at(temp);
                let n = (p - center) / self.radius;
                result = HitRecord::check_alpha_and_create(
                    ray,
                    temp,
                    TextureCoordinate::from_sphere(&n),
                    p,
                    n,
                    self.material.clone(),
                );
            }
            if result.is_none() {
                temp = (-b + FSize::sqrt(discriminant)) / (2.0 * a);
                if t_range.contains(&temp) {
                    let p = ray.point_at(temp);
                    let n = (p - center) / self.radius;
                    result = HitRecord::check_alpha_and_create(
                        ray,
                        temp,
                        TextureCoordinate::from_sphere(&n),
                        p,
                        n,
                        self.material.clone(),
                    );
                }
            }
            return result;
        }
        None
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        match self.hit(&Ray::new_ray(*o, *v), 0.001..FSize::MAX) {
            Some(_) => {
                let center = self.center(random::generate_size());
                let cos_theta_max = FSize::sqrt(
                    1.0 - self.radius * self.radius / glm::dot(center - *o, center - *o),
                );
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            None => 0.0,
        }
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        let center = self.center(random::generate_size());
        let direction = center - *o;
        let distance_squared = glm::dot(direction, direction);
        let own = OrthoNormalBase::form_w(&direction);
        own.local(random::generate_to_sphere(self.radius, distance_squared))
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_shape_movable_sphere(&self)
    }
}

#[cfg(test)]
mod movable_sphere_test {
    use super::*;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;

    #[test]
    fn bounding_box_test() {
        let s = MovableSphere::new(
            Point3::new(0.0, 0.0, 0.0)..Point3::new(0.0, 0.0, 1.0),
            0.0..1.0,
            1.0,
            Arc::new(NoMaterial::new()),
        );
        let b = s.bounding_box(0.0..1.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(-1.0, -1.0, -1.0), 0.1);
                test::assert_eq_vector3(&b.max, &Vector3::new(1.0, 1.0, 2.0), 0.1);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn hit_test() {
        let s = MovableSphere::new(
            Point3::new(0.0, 0.0, 0.0)..Point3::new(0.0, 0.0, 1.0),
            0.0..1.0,
            1.0,
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            )))),
        );
        let ray1 = Ray::new_ray(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match s.hit(&ray1, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match s.hit(&ray1, 10.0..20.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match s.hit(&ray2, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
