use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::collection::LeafNode;
use crate::geometry::{Geometry, Visitor};
use crate::math::{Ray, AABB};
use crate::random;
use crate::types::{FSize, Vector3};
use std::cmp::Ordering;
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

/// Bounding volume hierarchy node  
pub struct BVHNode {
    pub id: usize,
    pub left: Arc<dyn Geometry>,
    pub right: Arc<dyn Geometry>,
    bounding_box: Option<AABB>,
}

impl BVHNode {
    pub fn new(list: &Vec<Arc<dyn Geometry>>, t_range: Range<FSize>) -> Arc<dyn Geometry> {
        BVHNode::new_id(Object::new_id(), list, t_range)
    }

    pub fn new_id(
        id: usize,
        list: &Vec<Arc<dyn Geometry>>,
        t_range: Range<FSize>,
    ) -> Arc<dyn Geometry> {
        if list.len() == 1 {
            return Arc::new(LeafNode::new(list[0].clone(), t_range));
        }

        let axis = random::generate_axis();
        let mut list = list.clone();
        list.sort_by(|a, b| {
            let ba = a.bounding_box(t_range.clone());
            let bb = b.bounding_box(t_range.clone());
            match (ba, bb) {
                (Some(ba), Some(bb)) => {
                    if ba.min[axis] < bb.min[axis] {
                        Ordering::Less
                    } else if ba.min[axis] > bb.min[axis] {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
                (_, _) => Ordering::Equal,
            }
        });
        let left = BVHNode::new(&list[0..list.len() / 2].to_vec(), t_range.clone());
        let right = BVHNode::new(
            &list[(list.len() / 2)..list.len()].to_vec(),
            t_range.clone(),
        );
        let bounding_box = BVHNode::new_bounding_box(&left, &right, t_range);
        Arc::new(BVHNode {
            id,
            left,
            right,
            bounding_box: bounding_box,
        })
    }

    fn new_bounding_box(
        left: &Arc<dyn Geometry>,
        right: &Arc<dyn Geometry>,
        t_range: Range<FSize>,
    ) -> Option<AABB> {
        let box_left = left.bounding_box(t_range.clone());
        let box_right = right.bounding_box(t_range);
        match (box_left, box_right) {
            (Some(box_left), Some(box_right)) => Some(box_left | box_right),
            (Some(box_left), _) => Some(box_left),
            (_, Some(box_right)) => Some(box_right),
            (_, _) => None,
        }
    }
}

impl Geometry for BVHNode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, _: Range<FSize>) -> Option<AABB> {
        self.bounding_box.clone()
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        match &self.bounding_box {
            Some(bounding_box) => {
                if !bounding_box.hit(&ray, t_range.clone()) {
                    return None;
                }
            }
            None => (),
        }

        // TODO: test the "nearer" leaf first
        match self.left.hit(ray, t_range.clone()) {
            Some(left_hit) => {
                let right_hit = self.right.hit(ray, t_range.start..left_hit.t);
                match right_hit {
                    Some(right_hit) => Some(right_hit),
                    _ => Some(left_hit),
                }
            },
            _ => {
                let right_hit = self.right.hit(ray, t_range);
                match right_hit {
                    Some(right_hit) => Some(right_hit),
                    _ => None,
                }
            }
        }
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        0.5 * self.left.pdf_value(o, v) + 0.5 * self.right.pdf_value(o, v)
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        if random::generate_size() < 0.5 {
            self.left.random(o)
        } else {
            self.right.random(o)
        }
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_collection_bvh_node(&self)
    }
}

#[cfg(test)]
mod bhv_node_test {
    use super::*;
    use crate::geometry::shape::Sphere;
    use crate::material::{Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, Vector3};

    #[test]
    fn bounding_box_test() {
        let hl = BVHNode::new(
            &vec![
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
            ],
            0.0..0.0,
        );
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
        let bvh = BVHNode::new(
            &vec![
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
            ],
            0.0..0.0,
        );
        let ray1 = Ray::new_ray(Vector3::new(-1.0, -5.0, -1.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(3.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match bvh.hit(&ray1, 0.0..10.0) {
            Some(_) => (),
            None => panic!("no result"),
        }
        match bvh.hit(&ray1, 10.0..20.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match bvh.hit(&ray2, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
