use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::{Geometry, Visitor};
use crate::math::{Ray, AABB};
use crate::types::{FSize, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct FlipNormals {
    pub id: usize,
    pub node: Arc<dyn Geometry>,
}

impl FlipNormals {
    pub fn new(node: Arc<dyn Geometry>) -> FlipNormals {
        FlipNormals {
            id: Object::new_id(),
            node: node.clone(),
        }
    }
}

impl Geometry for FlipNormals {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, time: Range<FSize>) -> Option<AABB> {
        self.node.bounding_box(time)
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        match self.node.hit(ray, t_range.clone()) {
            Some(mut hit_record) => {
                hit_record.invert_normal();
                Some(hit_record)
            }
            None => None,
        }
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.node.pdf_value(o, v)
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.node.random(o)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_instancing_flip_normals(&self)
    }
}

#[cfg(test)]
mod flip_normals_test {
    use super::*;
    use crate::geometry::shape::Sphere;
    use crate::material::{Metal, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use crate::types::{Point3, Vector3};

    #[test]
    fn bounding_box_test() {
        let s = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, Arc::new(NoMaterial::new()));
        let i = FlipNormals::new(Arc::new(s));
        let b = i.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(-1.0, -1.0, -1.0), 0.01);
                test::assert_eq_vector3(&b.max, &Vector3::new(1.0, 1.0, 1.0), 0.01);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn hit_test() {
        let s = Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(Metal::new(
                0.0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            )),
        );
        let i = FlipNormals::new(Arc::new(s));
        let ray1 = Ray::new_ray(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match i.hit(&ray1, 0.0..10.0) {
            Some(hit_record) => {
                test::assert_eq_vector3(&hit_record.normal, &Vector3::new(0.0, 1.0, 0.0), 0.01)
            }
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
