use crate::core::object::Object;
use crate::core::HitRecord;
use crate::hit_able::{HitAble, Visitor};
use crate::math::{Ray, AABB};
use crate::types::{FSize, Point3, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct RotateZ {
    pub id: usize,
    pub sin_theta: FSize,
    pub cos_theta: FSize,
    pub node: Arc<dyn HitAble>,
    aabb: Option<AABB>,
}

impl RotateZ {
    pub fn new(angle_radians: FSize, node: Arc<dyn HitAble>) -> RotateZ {
        RotateZ::new_id(Object::new_id(), angle_radians, node)
    }

    pub fn new_id(id: usize, angle_radians: FSize, node: Arc<dyn HitAble>) -> RotateZ {
        let mut r = RotateZ {
            id,
            sin_theta: angle_radians.sin(),
            cos_theta: angle_radians.cos(),
            node: node.clone(),
            aabb: node.bounding_box(0.0..1.0),
        };
        r.aabb = match r.aabb.clone() {
            Some(b) => Some(
                AABB::new(r.rotate(b.min), r.rotate(b.max))
                    .or_vector(r.rotate(Point3::new(b.min.x, b.max.y, b.min.z)))
                    .or_vector(r.rotate(Point3::new(b.max.x, b.min.y, b.max.z))),
            ),
            None => None,
        };
        r
    }

    fn rotate(&self, p: Point3) -> Point3 {
        Point3::new(
            self.cos_theta * p.y + self.sin_theta * p.x,
            -self.sin_theta * p.y + self.cos_theta * p.x,
            p.z,
        )
    }

    fn rotate_inverse(&self, p: Point3) -> Point3 {
        Point3::new(
            self.cos_theta * p.y - self.sin_theta * p.x,
            self.sin_theta * p.y + self.cos_theta * p.x,
            p.z,
        )
    }
}

impl HitAble for RotateZ {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, _: Range<FSize>) -> Option<AABB> {
        self.aabb.clone()
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        match self.node.hit(
            &Ray::new_ray_with_attributes(
                self.rotate_inverse(ray.origin),
                self.rotate_inverse(ray.direction),
                ray,
            ),
            t_range,
        ) {
            Some(mut hit_record) => {
                hit_record.position = self.rotate(hit_record.position);
                hit_record.normal = self.rotate(hit_record.normal);
                Some(hit_record)
            }
            None => None,
        }
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.node.pdf_value(o, &self.rotate_inverse(*v))
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.node.random(o)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_instancing_rotate_z(&self)
    }
}

#[cfg(test)]
mod rotate_z_test {
    use super::*;
    use crate::hit_able::shape::Cuboid;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use crate::types::{Point3, Vector3};

    #[test]
    fn bounding_box_test() {
        let c = Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(NoMaterial::new()),
        );
        let i = RotateZ::new(FSize::to_radians(30.0), Arc::new(c));
        let b = i.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(-1.366, -1.366, -1.0), 0.01);
                test::assert_eq_vector3(&b.max, &Vector3::new(1.366, 1.366, 1.0), 0.01);
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
        let i = RotateZ::new(FSize::to_radians(30.0), Arc::new(c));
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
