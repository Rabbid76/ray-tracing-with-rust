use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::random;
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::sync::Arc;

/// Material node for blending materials
///
/// The materials are weighted and blended.
///
pub struct MaterialBlend {
    pub id: usize,
    pub materials: Vec<(FSize, Arc<dyn Material>)>,
    weights: Vec<FSize>,
}

impl MaterialBlend {
    pub fn new(materials: Vec<(FSize, Arc<dyn Material>)>) -> MaterialBlend {
        MaterialBlend::new_id(Object::new_id(), materials)
    }

    pub fn new_id(id: usize, materials: Vec<(FSize, Arc<dyn Material>)>) -> MaterialBlend {
        let weight_sum: FSize = materials.iter().map(|(w, _)| w).sum();
        let mut weights = Vec::<FSize>::default();
        let mut sum = 0.0;
        for i in 0..materials.len() {
            sum += materials[i].0;
            weights.push(sum / weight_sum);
        }
        MaterialBlend {
            id: id,
            materials,
            weights,
        }
    }
}

impl Material for MaterialBlend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn material(&self) -> Option<Arc<dyn Material>> {
        let random_weight = random::generate_size();
        let mut i = 0;
        while i < self.weights.len() - 1 && random_weight > self.weights[i] {
            i += 1;
        }
        Some(self.materials[i].1.clone())
    }

    fn scatter(&self, _: Arc<dyn Material>, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        panic!("internal error")
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> FSize {
        panic!("internal error")
    }

    fn has_alpha(&self) -> bool {
        panic!("internal error")
    }

    fn emitted(&self, _: &Ray, _: &HitRecord) -> ColorRGB {
        panic!("internal error")
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_material_blend(&self)
    }
}

#[cfg(test)]
mod material_blend_test {}
