use crate::core::object::Object;
use crate::core::HitRecord;
use crate::hit_able::collection::HitAbleList;
use crate::hit_able::instancing::FlipNormals;
use crate::hit_able::shape::{XYRect, XZRect, YZRect};
use crate::hit_able::{HitAble, Visitor};
use crate::material::Material;
use crate::math::{Ray, AABB};
use crate::types::{FSize, Point3, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct Cuboid {
    pub id: usize,
    pub aabb: AABB,
    pub material: Arc<dyn Material>,
    pub sides: HitAbleList,
}

impl Cuboid {
    pub fn new(aabb: Range<Point3>, material: Arc<dyn Material>) -> Cuboid {
        Cuboid::new_id(Object::new_id(), aabb, material)
    }

    pub fn new_id(id: usize, aabb: Range<Point3>, material: Arc<dyn Material>) -> Cuboid {
        let b = AABB::new(aabb.start, aabb.end);
        let sides: Vec<Arc<dyn HitAble>> = vec![
            Arc::new(XYRect::new(
                (b.min.x, b.min.y)..(b.max.x, b.max.y),
                b.max.z,
                material.clone(),
            )),
            Arc::new(FlipNormals::new(Arc::new(XYRect::new(
                (b.min.x, b.min.y)..(b.max.x, b.max.y),
                b.min.z,
                material.clone(),
            )))),
            Arc::new(XZRect::new(
                (b.min.x, b.min.z)..(b.max.x, b.max.z),
                b.max.y,
                material.clone(),
            )),
            Arc::new(FlipNormals::new(Arc::new(XZRect::new(
                (b.min.x, b.min.z)..(b.max.x, b.max.z),
                b.min.y,
                material.clone(),
            )))),
            Arc::new(YZRect::new(
                (b.min.y, b.min.z)..(b.max.y, b.max.z),
                b.max.x,
                material.clone(),
            )),
            Arc::new(FlipNormals::new(Arc::new(YZRect::new(
                (b.min.y, b.min.z)..(b.max.y, b.max.z),
                b.min.x,
                material.clone(),
            )))),
        ];
        Cuboid {
            id,
            aabb: b,
            material,
            sides: HitAbleList::new(&sides),
        }
    }
}

impl HitAble for Cuboid {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, _: Range<FSize>) -> Option<AABB> {
        Some(self.aabb.clone())
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        self.sides.hit(ray, t_range)
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.sides.pdf_value(o, v)
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.sides.random(o)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_shape_cuboid(&self)
    }
}

#[cfg(test)]
mod cuboid_test {
    use super::*;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use crate::types::Vector3;

    #[test]
    fn bounding_box_test() {
        let r = Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(NoMaterial::new()),
        );
        let b = r.bounding_box(0.0..0.0);
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
        let r = Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            )))),
        );
        let ray1 = Ray::new_ray(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(0.0, -2.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray3 = Ray::new_ray(Vector3::new(0.0, 0.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        match r.hit(&ray1, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match r.hit(&ray2, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match r.hit(&ray3, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
    }
}
