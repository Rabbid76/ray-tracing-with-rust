use crate::types::{FSize, Vector3};

mod cosine_pdf;
pub use self::cosine_pdf::CosinePdf;

mod hit_able_pdf;
pub use self::hit_able_pdf::HitAblePdf;

mod mixture_pdf;
pub use self::mixture_pdf::MixturePdf;

pub trait ProbabilityDensityFunction {
    fn value(&self, direction: &Vector3) -> FSize;
    fn generate(&self) -> Vector3;
}
