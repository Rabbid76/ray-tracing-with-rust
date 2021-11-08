use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::{Geometry, Visitor};
use crate::math::{Ray, AABB};
use crate::types::{FSize, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct Translate {
    pub id: usize,
    pub offset: Vector3,
    pub node: Arc<dyn Geometry>,
}

impl Translate {
    pub fn new(offset: Vector3, node: Arc<dyn Geometry>) -> Translate {
        Translate::new_id(Object::new_id(), offset, node)
    }

    pub fn new_id(id: usize, offset: Vector3, node: Arc<dyn Geometry>) -> Translate {
        Translate {
            id,
            offset,
            node: node.clone(),
        }
    }
}

impl Geometry for Translate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, time: Range<FSize>) -> Option<AABB> {
        match self.node.bounding_box(time) {
            Some(aabb) => Some(AABB::new(aabb.min + self.offset, aabb.max + self.offset)),
            None => None,
        }
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        match self.node.hit(
            &Ray::new_ray_with_attributes(ray.origin - self.offset, ray.direction, ray),
            t_range,
        ) {
            Some(mut hit_record) => {
                hit_record.displace(self.offset);
                Some(hit_record)
            }
            None => None,
        }
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.node.pdf_value(&(*o - self.offset), v)
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.node.random(&(*o - self.offset))
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_instancing_translate(&self)
    }
}

#[cfg(test)]
mod translate_test {
    use super::*;
    use crate::geometry::shape::Cuboid;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use crate::types::Point3;

    #[test]
    fn bounding_box_test() {
        let c = Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(NoMaterial::new()),
        );
        let i = Translate::new(Vector3::new(1.0, 1.0, 1.0), Arc::new(c));
        let b = i.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(0.0, 0.0, 0.0), 0.01);
                test::assert_eq_vector3(&b.max, &Vector3::new(2.0, 2.0, 2.0), 0.01);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn hit_test() {
        let c = Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            )))),
        );
        let i = Translate::new(Vector3::new(-0.5, 0.0, 0.0), Arc::new(c));
        let ray1 = Ray::new_ray(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match i.hit(&ray1, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match i.hit(&ray1, 10.0..20.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match i.hit(&ray2, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
