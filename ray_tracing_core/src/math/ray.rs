use crate::types::{FSize, Point3, Time, Vector3};

/// Ray object
///
/// Object that represents a ray in the scene. Stores the equation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: Time,
    pub w: Option<FSize>,
}

impl Ray {
    pub fn new_ray(origin: Point3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction,
            time: 0.0,
            w: None,
        }
    }

    pub fn new_ray_with_attributes(origin: Point3, direction: Vector3, ray: &Ray) -> Ray {
        Ray {
            origin,
            direction,
            time: ray.time,
            w: ray.w,
        }
    }

    pub fn new(origin: Point3, direction: Vector3, time: Time, w: Option<FSize>) -> Ray {
        Ray {
            origin,
            direction,
            time,
            w,
        }
    }

    pub fn point_at(&self, t: FSize) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod ray_test {
    use super::*;
    use crate::test;

    #[test]
    fn point_at_test() {
        let r = Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
        let p = r.point_at(1.0);
        test::assert_eq_vector3(&p, &Vector3::new(1.0, 1.0, 0.0), 0.0001);
    }
}
