use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::{Geometry, Visitor};
use crate::math::{Ray, AABB};
use crate::types::{FSize, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

/// Bounding volume leaf node  
pub struct LeafNode {
    pub id: usize,
    pub node: Arc<dyn Geometry>,
    bounding_box: Option<AABB>,
}

impl LeafNode {
    pub fn new(node: Arc<dyn Geometry>, t_range: Range<FSize>) -> LeafNode {
        LeafNode {
            id: Object::new_id(),
            node: node.clone(),
            bounding_box: node.bounding_box(t_range.clone()),
        }
    }
}

impl Geometry for LeafNode {
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
        self.node.hit(ray, t_range.clone())
    }

    fn pdf_value(&self, o: &Vector3, v: &Vector3) -> FSize {
        self.node.pdf_value(o, v)
    }

    fn random(&self, o: &Vector3) -> Vector3 {
        self.node.random(o)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_collection_leave_node(&self)
    }
}
