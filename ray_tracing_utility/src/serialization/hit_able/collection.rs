use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::hit_able::collection;
use ray_tracing_core::hit_able::collection::{BVHNode, HitAbleList};
use ray_tracing_core::hit_able::HitAble;
use ray_tracing_core::types::FSize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Collection {
    pub id: IdConstructor,

    #[serde(default = "Collection::default_bvh_tree")]
    pub bvh_tree: bool,

    pub object_id_list: Vec<IdReference>,
}

impl Collection {
    fn default_bvh_tree() -> bool {
        true
    }

    pub fn new(id: usize) -> Result<Collection, Box<dyn Error>> {
        Ok(Collection {
            id: IdConstructor::Single(id),
            bvh_tree: true,
            object_id_list: Vec::default(),
        })
    }

    pub fn from_list(l: &collection::HitAbleList) -> Result<Collection, Box<dyn Error>> {
        Ok(Collection {
            id: IdConstructor::Single(l.id),
            bvh_tree: false,
            object_id_list: l
                .list
                .iter()
                .map(|h| IdReference::Single(h.get_id()))
                .collect(),
        })
    }

    pub fn add(&mut self, h: Arc<dyn HitAble>) -> Result<(), Box<dyn Error>> {
        self.object_id_list.push(IdReference::Single(h.get_id()));
        Ok(())
    }

    pub fn to_collection(
        &self,
        list: &Vec<Arc<dyn HitAble>>,
        t_range: Range<FSize>,
    ) -> Result<Arc<dyn HitAble>, Box<dyn Error>> {
        if self.bvh_tree {
            Ok(Arc::new(HitAbleList::new(list)))
        } else {
            Ok(BVHNode::new(list, t_range))
        }
    }

    pub fn to_list(
        &self,
        list: &Vec<Arc<dyn HitAble>>,
    ) -> Result<Arc<dyn HitAble>, Box<dyn Error>> {
        Ok(Arc::new(HitAbleList::new(list)))
    }

    pub fn to_bvh_tree(
        &self,
        list: &Vec<Arc<dyn HitAble>>,
        t_range: Range<FSize>,
    ) -> Result<Arc<dyn HitAble>, Box<dyn Error>> {
        Ok(BVHNode::new(list, t_range))
    }
}

#[cfg(test)]
mod collection_test {
    use super::*;
    use ray_tracing_core::hit_able::shape;
    use ray_tracing_core::material;
    use ray_tracing_core::types::Point3;

    #[test]
    fn collection_test_form_list() {
        let m = Arc::new(material::NoMaterial::new());
        let s1 = Arc::new(shape::Sphere::new(
            Point3::new(-1.0, 0.0, 0.0),
            1.0,
            m.clone(),
        ));
        let s1_id = s1.id;
        let s2 = Arc::new(shape::Sphere::new(
            Point3::new(1.0, 0.0, 0.0),
            1.0,
            m.clone(),
        ));
        let s2_id = s2.id;
        let l = Arc::new(collection::HitAbleList::new(&vec![s1, s2]));
        let c = Collection::from_list(&l).unwrap();
        assert_eq!(c.object_id_list.len(), 2);
        assert_eq!(c.object_id_list[0], IdReference::Single(s1_id));
        assert_eq!(c.object_id_list[1], IdReference::Single(s2_id));
    }

    // TODO to_list test

    // TODO to_bvh_tree test

    // TODO test
}
