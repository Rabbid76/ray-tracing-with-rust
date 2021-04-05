use crate::serialization::IdConstructor;
use ray_tracing_core::material;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NoMaterial {
    pub id: IdConstructor,
}

impl NoMaterial {
    pub fn from_material(m: &material::NoMaterial) -> Result<NoMaterial, Box<dyn Error>> {
        Ok(NoMaterial {
            id: IdConstructor::Single(m.id),
        })
    }

    pub fn to_material(&self, index: usize) -> Result<material::NoMaterial, Box<dyn Error>> {
        Ok(material::NoMaterial {
            id: self.id.get_id(index),
        })
    }
}

#[cfg(test)]
mod no_material_test {
    use super::*;

    #[test]
    fn no_material_test_form_material() {
        let m = material::NoMaterial::new();
        let nm = NoMaterial::from_material(&m).unwrap();
        assert_eq!(
            nm,
            NoMaterial {
                id: IdConstructor::Single(m.id),
            }
        );
    }

    #[test]
    fn no_material_test_to_material() {
        let nm = NoMaterial {
            id: IdConstructor::Single(0),
        };
        let m = nm.to_material(0).unwrap();
        assert_eq!(
            NoMaterial::from_material(&m).unwrap(),
            NoMaterial {
                id: IdConstructor::Single(0)
            }
        );
    }
}
