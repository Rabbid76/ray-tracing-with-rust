use crate::math::OrthoNormalBase;
use crate::probability_density_function::ProbabilityDensityFunction;
use crate::random;
use crate::types::{FSize, Vector3};
use std::f64::consts::PI;

pub struct CosinePdf {
    pub ortho_normal_base: OrthoNormalBase,
}

impl CosinePdf {
    pub fn from_w(n: &Vector3) -> CosinePdf {
        CosinePdf {
            ortho_normal_base: OrthoNormalBase::form_w(n),
        }
    }
}

impl ProbabilityDensityFunction for CosinePdf {
    fn value(&self, direction: &Vector3) -> FSize {
        let n_dot_d = glm::dot(glm::normalize(*direction), self.ortho_normal_base.w());
        if n_dot_d < 0.0 {
            0.0
        } else {
            n_dot_d / PI
        }
    }

    fn generate(&self) -> Vector3 {
        self.ortho_normal_base
            .local(random::generate_cosine_direction())
    }
}
