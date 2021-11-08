use crate::geometry::Geometry;
use crate::probability_density_function::ProbabilityDensityFunction;
use crate::types::{FSize, Vector3};
use std::sync::Arc;

pub struct GeometryPdf {
    pub origin: Vector3,
    pub geometry: Arc<dyn Geometry>,
}

impl GeometryPdf {
    pub fn new(origin: &Vector3, geometry: Arc<dyn Geometry>) -> GeometryPdf {
        GeometryPdf {
            origin: *origin,
            geometry,
        }
    }
}

impl ProbabilityDensityFunction for GeometryPdf {
    fn value(&self, direction: &Vector3) -> FSize {
        self.geometry.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vector3 {
        self.geometry.random(&self.origin)
    }
}
