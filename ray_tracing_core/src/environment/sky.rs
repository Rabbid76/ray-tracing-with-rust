use crate::core::object::Object;
use crate::environment::{Environment, Visitor};
use crate::math::Ray;
use crate::types::ColorRGB;
use std::error::Error;

pub struct Sky {
    pub id: usize,
    pub nadir_color: ColorRGB,
    pub zenith_color: ColorRGB,
}

impl Sky {
    pub fn new(nadir_color: ColorRGB, zenith_color: ColorRGB) -> Sky {
        Sky {
            id: Object::new_id(),
            nadir_color,
            zenith_color,
        }
    }
}

impl Environment for Sky {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get(&self, ray: &Ray) -> ColorRGB {
        let unit_direction = glm::normalize(ray.direction);
        let t = unit_direction.y * 0.5 + 0.5;
        self.nadir_color * (1.0 - t) + self.zenith_color * t
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_sky(&self)
    }
}

#[cfg(test)]
mod sky_test {
    use super::*;
    use crate::test;
    use crate::types::{Point3, Vector3};

    #[test]
    fn get_test() {
        let s = Sky::new(ColorRGB::new(1.0, 0.0, 0.0), ColorRGB::new(1.0, 1.0, 1.0));
        let r = Ray::new_ray(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let c = s.get(&r);
        test::assert_eq_vector3(&c, &Vector3::new(1.0, 0.5, 0.5), 0.001);
    }
}
