use crate::serialization::IdConstructor;
use ray_tracing_core::core;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    pub id: IdConstructor,
    pub maximum_depth: usize,
}

impl Configuration {
    pub fn from_configuration(c: &core::Configuration) -> Result<Configuration, Box<dyn Error>> {
        Ok(Configuration {
            id: IdConstructor::Single(c.id),
            maximum_depth: c.maximum_depth,
        })
    }

    pub fn to_configuration(&self, index: usize) -> Result<core::Configuration, Box<dyn Error>> {
        Ok(core::Configuration {
            id: self.id.get_id(index),
            maximum_depth: self.maximum_depth,
        })
    }
}

#[cfg(test)]
mod configuration_test {
    use super::*;

    #[test]
    fn configuration_form_configuration() {
        let cc = core::Configuration::new(100);
        let c = Configuration::from_configuration(&cc).unwrap();
        assert_eq!(cc.maximum_depth, c.maximum_depth);
    }

    #[test]
    fn configuration_to_configuration() {
        let c = Configuration {
            id: IdConstructor::Single(0),
            maximum_depth: 100,
        };
        let cc = c.to_configuration(0).unwrap();
        assert_eq!(c.maximum_depth, cc.maximum_depth);
    }
}
