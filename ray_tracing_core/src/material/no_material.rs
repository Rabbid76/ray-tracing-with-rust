use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::sync::Arc;

/// No material (mockup material)
pub struct NoMaterial {
    pub id: usize,
}

impl NoMaterial {
    pub fn new() -> NoMaterial {
        NoMaterial {
            id: Object::new_id(),
        }
    }
}

impl Material for NoMaterial {
    fn get_id(&self) -> usize {
        self.id
    }

    fn scatter(
        &self,
        _self_material: Arc<dyn Material>,
        _: &Ray,
        _: &HitRecord,
    ) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> FSize {
        1.0
    }

    fn has_alpha(&self) -> bool {
        true
    }

    fn emitted(&self, _: &Ray, _: &HitRecord) -> ColorRGB {
        ColorRGB::new(0.0, 0.0, 0.0)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_no_material(&self)
    }
}

#[cfg(test)]
mod no_material_test {
    use super::*;
    use crate::test;
    use crate::types::{Point3, Vector3};

    #[test]
    fn scatter_test() {
        let m = Arc::new(NoMaterial::new());
        let result = m.scatter(
            m.clone(),
            &Ray::new_ray(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &mut HitRecord::empty(),
        );
        match result {
            None => (),
            _ => panic!("expected None"),
        }
    }

    #[test]
    fn emitted_test() {
        let m = NoMaterial::new();
        let c = m.emitted(
            &Ray::new_ray(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::empty(),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(0.0, 0.0, 0.0), 0.01);
    }
}
