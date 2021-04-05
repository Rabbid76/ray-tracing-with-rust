use crate::probability_density_function::ProbabilityDensityFunction;
use crate::random;
use crate::types::{FSize, Vector3};
use std::sync::Arc;

pub struct MixturePdf {
    // TODO Vec<Arc<dyn ProbabilityDensityFunction>>
    pub pdf_0: Arc<dyn ProbabilityDensityFunction>,
    pub pdf_1: Arc<dyn ProbabilityDensityFunction>,
}

impl MixturePdf {
    pub fn new(
        pdf_0: Arc<dyn ProbabilityDensityFunction>,
        pdf_1: Arc<dyn ProbabilityDensityFunction>,
    ) -> MixturePdf {
        MixturePdf {
            pdf_0: pdf_0.clone(),
            pdf_1: pdf_1.clone(),
        }
    }
}

impl ProbabilityDensityFunction for MixturePdf {
    fn value(&self, direction: &Vector3) -> FSize {
        0.5 * self.pdf_0.value(direction) + 0.5 * self.pdf_1.value(direction)
    }

    fn generate(&self) -> Vector3 {
        if random::generate_size() < 0.5 {
            self.pdf_0.generate()
        } else {
            self.pdf_1.generate()
        }
    }
}
