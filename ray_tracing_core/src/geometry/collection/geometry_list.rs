use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::{Geometry, Visitor};
use crate::math::{Ray, AABB};
use crate::random;
use crate::types::{FSize, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

/// List of hit able objects
pub struct GeometryList {
    pub id: usize,
    pub list: Vec<Arc<dyn Geometry>>,
}

impl GeometryList {
    pub fn new(list: &Vec<Arc<dyn Geometry>>) -> GeometryList {
        GeometryList {
            id: Object::new_id(),
            list: list.clone(),
        }
    }
}

impl Geometry for GeometryList {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, t_range: Range<FSize>) -> Option<AABB> {
        let mut outer_box = None;
        for geometry in self.list.iter() {
            outer_box = match (outer_box, geometry.bounding_box(t_range.clone())) {
                (Some(b1), Some(b2)) => Some(b1 | b2),
                (Some(b1), _) => Some(b1),
                (_, Some(b2)) => Some(b2),
                (_, _) => None,
            };
        }
        outer_box
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_range.end;
        for geometry in self.list.iter() {
            match geometry.hit(ray, t_range.start..closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    hit_record = Some(hit);
                }
                None => (),
            }
        }
        hit_record
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.list.iter().map(|node| node.pdf_value(o, v)).sum()
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.list[random::generate_from_range(0..self.list.len())].random(o)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_collection_geometry_list(&self)
    }
}

#[cfg(test)]
mod geometry_list_test {
    use super::*;
    use crate::geometry::shape::Sphere;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, Vector3};

    #[test]
    fn bounding_box_test() {
        let hl = GeometryList::new(&vec![
            Arc::new(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
            Arc::new(Sphere::new(
                Point3::new(1.0, 1.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        ]);
        let b = hl.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(-2.0, -1.0, -2.0), 0.1); // TODO -> 0.001
                test::assert_eq_vector3(&b.max, &Vector3::new(2.0, 2.0, 1.0), 0.1);
                // TODO -> 0.001
            }
            None => panic!("no bounding box"),
        }
    }

    #[test]
    fn hit_test() {
        let hl = GeometryList::new(&vec![
            Arc::new(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                1.0,
                Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                    ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
                )))),
            )),
            Arc::new(Sphere::new(
                Point3::new(1.0, 1.0, 0.0),
                1.0,
                Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                    ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
                )))),
            )),
        ]);
        let ray1 = Ray::new_ray(Vector3::new(-1.0, -5.0, -1.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(3.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match hl.hit(&ray1, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match hl.hit(&ray1, 10.0..20.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match hl.hit(&ray2, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
