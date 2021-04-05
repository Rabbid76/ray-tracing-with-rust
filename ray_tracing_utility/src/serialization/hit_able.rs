use crate::serialization::material::SerializeMaterial;
use crate::serialization::{IdReference, RayTracingObject};
use ray_tracing_core::hit_able;
use ray_tracing_core::material;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

/// Serialization of of ray tracing shapes
pub mod shape;

/// Serialization of hit able collections
pub mod collection;

/// Implementation of hit able volumes
pub mod volume;

/// Implementation of hit able instancing objects
pub mod instancing;

pub struct SerializeHitAble {
    pub object_map: Rc<RefCell<HashMap<usize, RayTracingObject>>>,
    pub collection: Option<(usize, collection::Collection)>,
}

impl SerializeHitAble {
    fn add_node(&mut self, h: Arc<dyn hit_able::HitAble>) -> Result<(), Box<dyn Error>> {
        h.accept(self)?;
        Ok(())
    }

    fn add_material(&mut self, t: Arc<dyn material::Material>) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&t.get_id()) {
            t.accept(&mut SerializeMaterial {
                object_map: self.object_map.clone(),
            })?;
        }
        Ok(())
    }

    fn add_hit_able(&mut self, h: Arc<dyn hit_able::HitAble>) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&h.get_id()) {
            h.accept(&mut SerializeHitAble {
                object_map: self.object_map.clone(),
                collection: None,
            })?;
        }
        Ok(())
    }

    fn create_collection(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        if self.collection == None {
            self.collection = Some((id, collection::Collection::new(id)?));
        }
        Ok(())
    }

    fn add_to_collection(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut c) = self.collection {
            c.1.object_id_list.push(IdReference::Single(id));
        };
        Ok(())
    }

    fn create_collection_object(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut c) = self.collection {
            self.object_map
                .borrow_mut()
                .insert(c.0, RayTracingObject::Collection(c.1.clone()));
            self.collection = None;
        }
        Ok(())
    }
}

impl Drop for SerializeHitAble {
    fn drop(&mut self) {
        self.create_collection_object().unwrap();
    }
}

impl hit_able::Visitor for SerializeHitAble {
    fn visit_collection_hit_able_list(
        &mut self,
        l: &hit_able::collection::HitAbleList,
    ) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&l.id) {
            for n in l.list.iter() {
                self.add_node(n.clone())?;
            }
            self.object_map.borrow_mut().insert(
                l.id,
                RayTracingObject::Collection(collection::Collection::from_list(l)?),
            );
        };
        Ok(())
    }

    fn visit_collection_bvh_node(
        &mut self,
        n: &hit_able::collection::BVHNode,
    ) -> Result<(), Box<dyn Error>> {
        self.create_collection(n.id)?;
        self.add_node(n.left.clone())?;
        self.add_node(n.right.clone())?;
        Ok(())
    }

    fn visit_collection_leave_node(
        &mut self,
        n: &hit_able::collection::LeafNode,
    ) -> Result<(), Box<dyn Error>> {
        self.create_collection(n.id)?;
        self.add_node(n.node.clone())?;
        Ok(())
    }

    fn visit_shape_sphere(&mut self, s: &hit_able::shape::Sphere) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(s.id)?;
        if !self.object_map.borrow().contains_key(&s.id) {
            self.add_material(s.material.clone())?;
            self.object_map.borrow_mut().insert(
                s.id,
                RayTracingObject::Sphere(shape::Sphere::from_shape(s)?),
            );
        };
        Ok(())
    }

    fn visit_shape_movable_sphere(
        &mut self,
        s: &hit_able::shape::MovableSphere,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(s.id)?;
        if !self.object_map.borrow().contains_key(&s.id) {
            self.add_material(s.material.clone())?;
            self.object_map.borrow_mut().insert(
                s.id,
                RayTracingObject::MovableSphere(shape::MovableSphere::from_shape(s)?),
            );
        };
        Ok(())
    }

    fn visit_shape_xy_rect(&mut self, r: &hit_able::shape::XYRect) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(r.id)?;
        if !self.object_map.borrow().contains_key(&r.id) {
            self.add_material(r.material.clone())?;
            self.object_map.borrow_mut().insert(
                r.id,
                RayTracingObject::XYRect(shape::XYRect::from_shape(r)?),
            );
        };
        Ok(())
    }

    fn visit_shape_xz_rect(&mut self, r: &hit_able::shape::XZRect) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(r.id)?;
        if !self.object_map.borrow().contains_key(&r.id) {
            self.add_material(r.material.clone())?;
            self.object_map.borrow_mut().insert(
                r.id,
                RayTracingObject::XZRect(shape::XZRect::from_shape(r)?),
            );
        };
        Ok(())
    }

    fn visit_shape_yz_rect(&mut self, r: &hit_able::shape::YZRect) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(r.id)?;
        if !self.object_map.borrow().contains_key(&r.id) {
            self.add_material(r.material.clone())?;
            self.object_map.borrow_mut().insert(
                r.id,
                RayTracingObject::YZRect(shape::YZRect::from_shape(r)?),
            );
        };
        Ok(())
    }

    fn visit_shape_cuboid(&mut self, c: &hit_able::shape::Cuboid) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(c.id)?;
        if !self.object_map.borrow().contains_key(&c.id) {
            self.add_material(c.material.clone())?;
            self.object_map.borrow_mut().insert(
                c.id,
                RayTracingObject::Cuboid(shape::Cuboid::from_shape(c)?),
            );
        };
        Ok(())
    }

    fn visit_volume_constant_medium(
        &mut self,
        v: &hit_able::volume::ConstantMedium,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(v.id)?;
        if !self.object_map.borrow().contains_key(&v.id) {
            self.add_hit_able(v.boundary.clone())?;
            self.add_material(v.phase_function.clone())?;
            self.object_map.borrow_mut().insert(
                v.id,
                RayTracingObject::ConstantMedium(volume::ConstantMedium::from_volume(v)?),
            );
        };
        Ok(())
    }

    fn visit_instancing_flip_normals(
        &mut self,
        i: &hit_able::instancing::FlipNormals,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(i.id)?;
        if !self.object_map.borrow().contains_key(&i.id) {
            self.add_hit_able(i.node.clone())?;
            self.object_map.borrow_mut().insert(
                i.id,
                RayTracingObject::FlipNormals(instancing::FlipNormals::from_hit_able(i)?),
            );
        };
        Ok(())
    }

    fn visit_instancing_rotate_x(
        &mut self,
        i: &hit_able::instancing::RotateX,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(i.id)?;
        if !self.object_map.borrow().contains_key(&i.id) {
            self.add_hit_able(i.node.clone())?;
            self.object_map.borrow_mut().insert(
                i.id,
                RayTracingObject::RotateX(instancing::RotateX::from_hit_able(i)?),
            );
        };
        Ok(())
    }

    fn visit_instancing_rotate_y(
        &mut self,
        i: &hit_able::instancing::RotateY,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(i.id)?;
        if !self.object_map.borrow().contains_key(&i.id) {
            self.add_hit_able(i.node.clone())?;
            self.object_map.borrow_mut().insert(
                i.id,
                RayTracingObject::RotateY(instancing::RotateY::from_hit_able(i)?),
            );
        };
        Ok(())
    }

    fn visit_instancing_rotate_z(
        &mut self,
        i: &hit_able::instancing::RotateZ,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(i.id)?;
        if !self.object_map.borrow().contains_key(&i.id) {
            self.add_hit_able(i.node.clone())?;
            self.object_map.borrow_mut().insert(
                i.id,
                RayTracingObject::RotateZ(instancing::RotateZ::from_hit_able(i)?),
            );
        };
        Ok(())
    }

    fn visit_instancing_translate(
        &mut self,
        i: &hit_able::instancing::Translate,
    ) -> Result<(), Box<dyn Error>> {
        self.add_to_collection(i.id)?;
        if !self.object_map.borrow().contains_key(&i.id) {
            self.add_hit_able(i.node.clone())?;
            self.object_map.borrow_mut().insert(
                i.id,
                RayTracingObject::Translate(instancing::Translate::from_hit_able(i)?),
            );
        };
        Ok(())
    }
}

#[cfg(test)]
mod serialize_hit_able_test {
    use super::*;
    use ray_tracing_core::hit_able::HitAble;
    use ray_tracing_core::texture;
    use ray_tracing_core::types::{ColorRGBA, Point3};

    #[test]
    fn serialize_test_unique_objects() {
        let m1 = Arc::new(material::NoMaterial::new());
        let s1 = Arc::new(hit_able::shape::Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            4.0,
            m1.clone(),
        ));
        let t2 = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            1.0, 0.5, 0.0, 1.0,
        )));
        let m2 = Arc::new(material::Lambertian::new(t2.clone()));
        let s2 = Arc::new(hit_able::shape::Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            4.0,
            m2.clone(),
        ));
        let b = hit_able::collection::BVHNode::new(&vec![s1.clone(), s2.clone()], 0.0..0.0);
        let objects = Rc::new(RefCell::new(HashMap::<usize, RayTracingObject>::default()));
        let mut s = SerializeHitAble {
            object_map: objects.clone(),
            collection: None,
        };
        b.accept(&mut s).unwrap();
        drop(s);
        assert_eq!(objects.borrow_mut().len(), 6);
        match &objects.borrow_mut()[&m1.id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&s1.id] {
            RayTracingObject::Sphere(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&t2.id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&m2.id] {
            RayTracingObject::Lambertian(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&s2.id] {
            RayTracingObject::Sphere(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&b.get_id()] {
            RayTracingObject::Collection(c) => {
                assert_eq!(c.object_id_list.len(), 2);
            }
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn serialize_test_shared_objects() {
        let m1 = Arc::new(material::NoMaterial::new());
        let s1 = Arc::new(hit_able::shape::Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            4.0,
            m1.clone(),
        ));
        let b =
            hit_able::collection::BVHNode::new(&vec![s1.clone(), s1.clone(), s1.clone()], 0.0..0.0);
        let objects = Rc::new(RefCell::new(HashMap::<usize, RayTracingObject>::default()));
        let mut s = SerializeHitAble {
            object_map: objects.clone(),
            collection: None,
        };
        b.accept(&mut s).unwrap();
        drop(s);
        assert_eq!(objects.borrow_mut().len(), 3);
        match &objects.borrow_mut()[&m1.id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&s1.id] {
            RayTracingObject::Sphere(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&b.get_id()] {
            RayTracingObject::Collection(c) => {
                assert_eq!(c.object_id_list.len(), 3);
            }
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn serialize_test_shared_objects_in_list() {
        let m1 = Arc::new(material::NoMaterial::new());
        let s1 = Arc::new(hit_able::shape::Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            4.0,
            m1.clone(),
        ));
        let l = hit_able::collection::HitAbleList::new(&vec![s1.clone(), s1.clone()]);
        let objects = Rc::new(RefCell::new(HashMap::<usize, RayTracingObject>::default()));
        let mut s = SerializeHitAble {
            object_map: objects.clone(),
            collection: None,
        };
        l.accept(&mut s).unwrap();
        drop(s);
        assert_eq!(objects.borrow_mut().len(), 3);
        match &objects.borrow_mut()[&m1.id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&s1.id] {
            RayTracingObject::Sphere(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &objects.borrow_mut()[&l.id] {
            RayTracingObject::Collection(c) => {
                assert_eq!(c.object_id_list.len(), 2);
            }
            _ => panic!("unexpected ray tracing object"),
        };
    }
}
