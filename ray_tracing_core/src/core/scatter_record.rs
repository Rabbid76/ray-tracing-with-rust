use crate::material::Material;
use crate::math::Ray;
use crate::probability_density_function::ProbabilityDensityFunction;
use crate::types::{ColorRGB, FSize};
use std::sync::Arc;

#[derive(Clone)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub is_specular: bool,
    pub attenuation: ColorRGB,
    pub alpha: FSize,
    pub pdf: Option<Arc<dyn ProbabilityDensityFunction>>,
    pub material: Arc<dyn Material>,
}

impl ScatterRecord {
    pub fn new(
        ray: Ray,
        is_specular: bool,
        attenuation: ColorRGB,
        alpha: FSize,
        pdf: Option<Arc<dyn ProbabilityDensityFunction>>,
        material: Arc<dyn Material>,
    ) -> ScatterRecord {
        ScatterRecord {
            ray,
            is_specular: is_specular,
            attenuation,
            alpha,
            pdf,
            material,
        }
    }
}
