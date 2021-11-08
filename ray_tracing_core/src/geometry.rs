use crate::core::HitRecord;
use crate::math::{Ray, AABB};
use crate::types::{FSize, Vector3};
use std::error::Error;
use std::ops::Range;

/// Shape objects
///
/// Implementation of ray tracing shapes
pub mod shape;

/// Collections
///
/// Implementation of hit able collections
pub mod collection;

/// Volume
///
/// Implementation of hit able volumes
pub mod volume;

/// Instancing
///
/// Implementation of hit able instancing objects
pub mod instancing;

pub trait Geometry: Sync + Send {
    fn get_id(&self) -> usize;

    fn bounding_box(&self, t_range: Range<FSize>) -> Option<AABB>;

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord>;

    fn pdf_value(&self, _o: &Vector3, _v: &Vector3) -> FSize {
        0.0
    }

    fn random(&self, _o: &Vector3) -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor {
    fn visit_collection_geometry_list(
        &mut self,
        l: &collection::GeometryList,
    ) -> Result<(), Box<dyn Error>>;
    fn visit_collection_bvh_node(&mut self, n: &collection::BVHNode) -> Result<(), Box<dyn Error>>;
    fn visit_collection_leave_node(
        &mut self,
        n: &collection::LeafNode,
    ) -> Result<(), Box<dyn Error>>;
    fn visit_shape_sphere(&mut self, s: &shape::Sphere) -> Result<(), Box<dyn Error>>;
    fn visit_shape_movable_sphere(
        &mut self,
        s: &shape::MovableSphere,
    ) -> Result<(), Box<dyn Error>>;
    fn visit_shape_xy_rect(&mut self, r: &shape::XYRect) -> Result<(), Box<dyn Error>>;
    fn visit_shape_xz_rect(&mut self, r: &shape::XZRect) -> Result<(), Box<dyn Error>>;
    fn visit_shape_yz_rect(&mut self, r: &shape::YZRect) -> Result<(), Box<dyn Error>>;
    fn visit_shape_cuboid(&mut self, c: &shape::Cuboid) -> Result<(), Box<dyn Error>>;
    fn visit_volume_constant_medium(
        &mut self,
        v: &volume::ConstantMedium,
    ) -> Result<(), Box<dyn Error>>;
    fn visit_instancing_flip_normals(
        &mut self,
        i: &instancing::FlipNormals,
    ) -> Result<(), Box<dyn Error>>;
    fn visit_instancing_rotate_x(&mut self, i: &instancing::RotateX) -> Result<(), Box<dyn Error>>;
    fn visit_instancing_rotate_y(&mut self, i: &instancing::RotateY) -> Result<(), Box<dyn Error>>;
    fn visit_instancing_rotate_z(&mut self, i: &instancing::RotateZ) -> Result<(), Box<dyn Error>>;
    fn visit_instancing_translate(
        &mut self,
        i: &instancing::Translate,
    ) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod test_visitor {
    use super::*;
    use crate::material::NoMaterial;
    use crate::types::{Point3, Vector3};
    use std::sync::Arc;

    struct TestVisitor {
        pub count: Vec<usize>,
    }

    impl TestVisitor {
        pub fn default() -> TestVisitor {
            TestVisitor {
                count: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }

        pub fn evaluate(&self, index: usize, expected: usize) {
            for (i, count) in self.count.iter().enumerate() {
                assert_eq!(count, &if i == index { expected } else { 0 });
            }
        }
    }

    impl Visitor for TestVisitor {
        fn visit_collection_geometry_list(
            &mut self,
            l: &collection::GeometryList,
        ) -> Result<(), Box<dyn Error>> {
            for n in l.list.iter() {
                n.accept(self)?;
            }
            Ok(())
        }
        fn visit_collection_bvh_node(
            &mut self,
            n: &collection::BVHNode,
        ) -> Result<(), Box<dyn Error>> {
            n.left.accept(self)?;
            n.right.accept(self)
        }
        fn visit_collection_leave_node(
            &mut self,
            n: &collection::LeafNode,
        ) -> Result<(), Box<dyn Error>> {
            n.node.accept(self)
        }
        fn visit_shape_sphere(&mut self, _: &shape::Sphere) -> Result<(), Box<dyn Error>> {
            self.count[0] += 1;
            Ok(())
        }
        fn visit_shape_movable_sphere(
            &mut self,
            _: &shape::MovableSphere,
        ) -> Result<(), Box<dyn Error>> {
            self.count[1] += 1;
            Ok(())
        }
        fn visit_shape_xy_rect(&mut self, _: &shape::XYRect) -> Result<(), Box<dyn Error>> {
            self.count[2] += 1;
            Ok(())
        }
        fn visit_shape_xz_rect(&mut self, _: &shape::XZRect) -> Result<(), Box<dyn Error>> {
            self.count[3] += 1;
            Ok(())
        }
        fn visit_shape_yz_rect(&mut self, _: &shape::YZRect) -> Result<(), Box<dyn Error>> {
            self.count[4] += 1;
            Ok(())
        }
        fn visit_shape_cuboid(&mut self, _: &shape::Cuboid) -> Result<(), Box<dyn Error>> {
            self.count[5] += 1;
            Ok(())
        }
        fn visit_volume_constant_medium(
            &mut self,
            _: &volume::ConstantMedium,
        ) -> Result<(), Box<dyn Error>> {
            self.count[6] += 1;
            Ok(())
        }
        fn visit_instancing_flip_normals(
            &mut self,
            _: &instancing::FlipNormals,
        ) -> Result<(), Box<dyn Error>> {
            self.count[7] += 1;
            Ok(())
        }
        fn visit_instancing_rotate_x(
            &mut self,
            _: &instancing::RotateX,
        ) -> Result<(), Box<dyn Error>> {
            self.count[8] += 1;
            Ok(())
        }
        fn visit_instancing_rotate_y(
            &mut self,
            _: &instancing::RotateY,
        ) -> Result<(), Box<dyn Error>> {
            self.count[9] += 1;
            Ok(())
        }
        fn visit_instancing_rotate_z(
            &mut self,
            _: &instancing::RotateZ,
        ) -> Result<(), Box<dyn Error>> {
            self.count[10] += 1;
            Ok(())
        }
        fn visit_instancing_translate(
            &mut self,
            _: &instancing::Translate,
        ) -> Result<(), Box<dyn Error>> {
            self.count[11] += 1;
            Ok(())
        }
    }

    #[test]
    pub fn test_visitor_geometry_list() {
        let l = collection::GeometryList::new(&vec![
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        ]);
        let mut v = TestVisitor::default();
        l.accept(&mut v).unwrap();
        v.evaluate(0, 2);
    }

    #[test]
    pub fn test_visitor_bvh_node() {
        let l = collection::BVHNode::new(
            &vec![
                Arc::new(shape::Sphere::new(
                    Point3::new(0.0, 0.0, 0.0),
                    1.0,
                    Arc::new(NoMaterial::new()),
                )),
                Arc::new(shape::Sphere::new(
                    Point3::new(0.0, 0.0, 0.0),
                    1.0,
                    Arc::new(NoMaterial::new()),
                )),
            ],
            0.0..0.0,
        );
        let mut v = TestVisitor::default();
        l.accept(&mut v).unwrap();
        v.evaluate(0, 2);
    }

    #[test]
    pub fn test_visitor_leaf_node() {
        let l = collection::LeafNode::new(
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
            0.0..0.0,
        );
        let mut v = TestVisitor::default();
        l.accept(&mut v).unwrap();
        v.evaluate(0, 1);
    }

    #[test]
    pub fn test_visitor_sphere() {
        let s = shape::Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, Arc::new(NoMaterial::new()));
        let mut v = TestVisitor::default();
        s.accept(&mut v).unwrap();
        v.evaluate(0, 1);
    }

    #[test]
    pub fn test_visitor_movable_sphere() {
        let s = shape::MovableSphere::new(
            Point3::new(0.0, 0.0, 0.0)..Point3::new(0.0, 1.0, 0.0),
            0.0..1.0,
            1.0,
            Arc::new(NoMaterial::new()),
        );
        let mut v = TestVisitor::default();
        s.accept(&mut v).unwrap();
        v.evaluate(1, 1);
    }

    #[test]
    pub fn test_visitor_xy_rect() {
        let r = shape::XYRect::new((0.0, 0.0)..(1.0, 1.0), 0.5, Arc::new(NoMaterial::new()));
        let mut v = TestVisitor::default();
        r.accept(&mut v).unwrap();
        v.evaluate(2, 1);
    }

    #[test]
    pub fn test_visitor_xz_rect() {
        let r = shape::XZRect::new((0.0, 0.0)..(1.0, 1.0), 0.5, Arc::new(NoMaterial::new()));
        let mut v = TestVisitor::default();
        r.accept(&mut v).unwrap();
        v.evaluate(3, 1);
    }

    #[test]
    pub fn test_visitor_yz_rect() {
        let r = shape::YZRect::new((0.0, 0.0)..(1.0, 1.0), 0.5, Arc::new(NoMaterial::new()));
        let mut v = TestVisitor::default();
        r.accept(&mut v).unwrap();
        v.evaluate(4, 1);
    }

    #[test]
    pub fn test_visitor_cuboid() {
        let r = shape::Cuboid::new(
            Point3::new(-1.0, -1.0, -1.0)..Point3::new(1.0, 1.0, 1.0),
            Arc::new(NoMaterial::new()),
        );
        let mut v = TestVisitor::default();
        r.accept(&mut v).unwrap();
        v.evaluate(5, 1);
    }

    #[test]
    pub fn test_visitor_constant_medium() {
        // TODO [...]
    }

    #[test]
    pub fn test_visitor_flip_normals() {
        let i = instancing::FlipNormals::new(Arc::new(shape::Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(NoMaterial::new()),
        )));
        let mut v = TestVisitor::default();
        i.accept(&mut v).unwrap();
        v.evaluate(7, 1);
    }

    #[test]
    pub fn test_visitor_rotate_x() {
        let i = instancing::RotateX::new(
            FSize::to_radians(30.0),
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        );
        let mut v = TestVisitor::default();
        i.accept(&mut v).unwrap();
        v.evaluate(8, 1);
    }

    #[test]
    pub fn test_visitor_rotate_y() {
        let i = instancing::RotateY::new(
            FSize::to_radians(30.0),
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        );
        let mut v = TestVisitor::default();
        i.accept(&mut v).unwrap();
        v.evaluate(9, 1);
    }

    #[test]
    pub fn test_visitor_rotate_z() {
        let i = instancing::RotateZ::new(
            FSize::to_radians(30.0),
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        );
        let mut v = TestVisitor::default();
        i.accept(&mut v).unwrap();
        v.evaluate(10, 1);
    }

    #[test]
    pub fn test_visitor_translate() {
        let i = instancing::Translate::new(
            Vector3::new(1.0, 1.0, 1.0),
            Arc::new(shape::Sphere::new(
                Point3::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(NoMaterial::new()),
            )),
        );
        let mut v = TestVisitor::default();
        i.accept(&mut v).unwrap();
        v.evaluate(11, 1);
    }
}
