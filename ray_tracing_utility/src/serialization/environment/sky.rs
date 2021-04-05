use crate::serialization::{IdConstructor, Value};
use ray_tracing_core::environment;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sky {
    pub id: IdConstructor,
    pub nadir_color: Value,
    pub zenith_color: Value,
}

impl Sky {
    pub fn from_environment(s: &environment::Sky) -> Result<Sky, Box<dyn Error>> {
        Ok(Sky {
            id: IdConstructor::Single(s.id),
            nadir_color: Value::from_color_rgb(s.nadir_color)?,
            zenith_color: Value::from_color_rgb(s.zenith_color)?,
        })
    }

    pub fn to_environment(&self, index: usize) -> Result<environment::Sky, Box<dyn Error>> {
        Ok(environment::Sky {
            id: self.id.get_id(index),
            nadir_color: self.nadir_color.to_color_rgb()?,
            zenith_color: self.zenith_color.to_color_rgb()?,
        })
    }
}

#[cfg(test)]
mod sky_test {
    use super::*;
    use ray_tracing_core::environment;
    use ray_tracing_core::test;
    use ray_tracing_core::types::ColorRGB;

    #[test]
    fn sky_form_environment() {
        let s = environment::Sky::new(ColorRGB::new(1.0, 0.5, 0.0), ColorRGB::new(1.0, 1.0, 1.0));
        let sk = Sky::from_environment(&s).unwrap();
        assert_eq!(sk.nadir_color, Value::Vector3((1.0, 0.5, 0.0)));
    }

    #[test]
    fn sky_to_environment() {
        let sk = Sky {
            id: IdConstructor::Single(0),
            nadir_color: Value::Vector3((1.0, 0.5, 0.0)),
            zenith_color: Value::Vector3((1.0, 1.0, 1.0)),
        };
        let s = sk.to_environment(0).unwrap();
        test::assert_eq_vector3(&s.nadir_color, &ColorRGB::new(1.0, 0.5, 0.0), 0.001);
        test::assert_eq_vector3(&s.zenith_color, &ColorRGB::new(1.0, 1.0, 1.0), 0.001);
    }
}
