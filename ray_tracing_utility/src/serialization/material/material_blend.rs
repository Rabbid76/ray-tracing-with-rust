use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::material;
use ray_tracing_core::material::Material;
use ray_tracing_core::types::FSize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MaterialBlend {
    pub id: IdConstructor,
    pub weights: Vec<FSize>,
    pub materials: Vec<IdReference>,
}

impl MaterialBlend {
    pub fn from_material(m: &material::MaterialBlend) -> Result<MaterialBlend, Box<dyn Error>> {
        Ok(MaterialBlend {
            id: IdConstructor::Single(m.id),
            weights: m.materials.iter().map(|(w, _)| *w).collect(),
            materials: m
                .materials
                .iter()
                .map(|(_, m)| IdReference::Single(m.get_id()))
                .collect(),
        })
    }

    pub fn to_material(
        &self,
        index: usize,
        materials: &Vec<Arc<dyn Material>>,
    ) -> Result<material::MaterialBlend, Box<dyn Error>> {
        Ok(material::MaterialBlend::new_id(
            self.id.get_id(index),
            self.weights
                .iter()
                .zip(materials.iter())
                .map(|(w, m)| (*w, m.clone()))
                .collect(),
        ))
    }
}

#[cfg(test)]
mod metal_test {
    use super::*;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::ColorRGBA;

    #[test]
    fn metal_test_form_material() {
        let m1 = material::Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.5, 1.0, 1.0,
        ))));
        let m1_id = m1.id;
        let m2 = material::NoMaterial::new();
        let m2_id = m2.id;
        let m = material::MaterialBlend::new(vec![(2.0, Arc::new(m1)), (1.0, Arc::new(m2))]);
        let ma = MaterialBlend::from_material(&m).unwrap();
        assert_eq!(ma.materials.len(), 2);
        assert_eq!(ma.materials[0], IdReference::Single(m1_id));
        assert_eq!(ma.materials[1], IdReference::Single(m2_id));
        assert_eq!(ma.weights.len(), 2);
        assert_eq!(ma.weights[0], 2.0);
        assert_eq!(ma.weights[1], 1.0);
    }

    #[test]
    fn metal_test_to_material() {
        let ma = MaterialBlend {
            id: IdConstructor::Single(0),
            weights: vec![2.0, 1.0],
            materials: vec![IdReference::Single(1), IdReference::Single(2)],
        };
        let m = ma
            .to_material(
                0,
                &vec![
                    Arc::new(material::NoMaterial::new()),
                    Arc::new(material::NoMaterial::new()),
                ],
            )
            .unwrap();
        assert_eq!(m.materials.len(), 2);
    }
}
