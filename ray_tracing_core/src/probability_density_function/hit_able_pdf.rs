use crate::hit_able::HitAble;
use crate::probability_density_function::ProbabilityDensityFunction;
use crate::types::{FSize, Vector3};
use std::sync::Arc;

pub struct HitAblePdf {
    pub origin: Vector3,
    pub hit_able: Arc<dyn HitAble>,
}

impl HitAblePdf {
    pub fn new(origin: &Vector3, hit_able: Arc<dyn HitAble>) -> HitAblePdf {
        HitAblePdf {
            origin: *origin,
            hit_able,
        }
    }
}

impl ProbabilityDensityFunction for HitAblePdf {
    fn value(&self, direction: &Vector3) -> FSize {
        self.hit_able.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vector3 {
        self.hit_able.random(&self.origin)
    }
}
