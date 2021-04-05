use crate::math::Ray;
use crate::types::{FSize, Vector3};
use std::mem;
use std::ops::{BitOr, Range};

/// Axis aligned bounding box object
///
/// Object that represents an axially aligned bounding box in the scene.
/// Stores the minimum and maximum points of the object.
#[derive(Clone)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> AABB {
        AABB {
            min: Vector3::new(
                FSize::min(min.x, max.x),
                FSize::min(min.y, max.y),
                FSize::min(min.z, max.z),
            ),
            max: Vector3::new(
                FSize::max(min.x, max.x),
                FSize::max(min.y, max.y),
                FSize::max(min.z, max.z),
            ),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> bool {
        for a in 0..3 {
            //let t0 = FSize::min((self.min[a] - ray.origin[a]) / ray.direction[a], (self.max[a] - ray.origin[a]) / ray.direction[a]);
            //let t1 = FSize::max((self.min[a] - ray.origin[a]) / ray.direction[a], (self.max[a] - ray.origin[a]) / ray.direction[a]);

            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let tmin = FSize::max(t0, t_range.start);
            let tmax = FSize::min(t1, t_range.end);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn or_vector(&self, v: Vector3) -> AABB {
        AABB {
            min: Vector3::new(
                FSize::min(self.min.x, v.x),
                FSize::min(self.min.y, v.y),
                FSize::min(self.min.z, v.z),
            ),
            max: Vector3::new(
                FSize::max(self.max.x, v.x),
                FSize::max(self.max.y, v.y),
                FSize::max(self.max.z, v.z),
            ),
        }
    }
}

impl BitOr for AABB {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        AABB {
            min: Vector3::new(
                FSize::min(self.min.x, rhs.min.x),
                FSize::min(self.min.y, rhs.min.y),
                FSize::min(self.min.z, rhs.min.z),
            ),
            max: Vector3::new(
                FSize::max(self.max.x, rhs.max.x),
                FSize::max(self.max.y, rhs.max.y),
                FSize::max(self.max.z, rhs.max.z),
            ),
        }
    }
}

#[cfg(test)]
mod aabb_test {
    use super::*;
    use crate::test;

    #[test]
    fn new_test() {
        let b = AABB::new(Vector3::new(5.0, 3.0, 1.0), Vector3::new(2.0, 4.0, 6.0));
        test::assert_eq_vector3(&b.min, &Vector3::new(2.0, 3.0, 1.0), 0.001);
        test::assert_eq_vector3(&b.max, &Vector3::new(5.0, 4.0, 6.0), 0.001);
    }

    #[test]
    fn bitor_test() {
        let b1 = AABB::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 4.0, 6.0));
        let b2 = AABB::new(Vector3::new(5.0, 3.0, 1.0), Vector3::new(6.0, 6.0, 6.0));
        let b_or = b1 | b2;
        test::assert_eq_vector3(&b_or.min, &Vector3::new(1.0, 1.0, 1.0), 0.001);
        test::assert_eq_vector3(&b_or.max, &Vector3::new(6.0, 6.0, 6.0), 0.001);
    }

    #[test]
    fn hit_test() {
        let b = AABB::new(Vector3::new(-1.0, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0));
        let ray1 = Ray::new_ray(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(b.hit(&ray1, 0.0..10.0));
        assert!(!b.hit(&ray1, 10.0..20.0));
        assert!(!b.hit(&ray2, 0.0..10.0));
    }
}
