use crate::serialization::{IdConstructor, Value};
use ray_tracing_core::core;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Camera {
    pub id: IdConstructor,
    pub lower_left_corner: Value,
    pub horizontal: Value,
    pub vertical: Value,
    pub origin: Value,
    pub lense_radius: Value,
    pub time_from: Value,
    pub time_to: Value,
}

impl Camera {
    pub fn from_camera(c: &core::Camera) -> Result<Camera, Box<dyn Error>> {
        Ok(Camera {
            id: IdConstructor::Single(c.id),
            lower_left_corner: Value::from_vector3(c.lower_left_corner)?,
            horizontal: Value::from_vector3(c.horizontal)?,
            vertical: Value::from_vector3(c.vertical)?,
            origin: Value::from_vector3(c.origin)?,
            lense_radius: Value::from_value(c.lense_radius)?,
            time_from: Value::from_value(c.time.start)?,
            time_to: Value::from_value(c.time.end)?,
        })
    }

    pub fn to_camera(&self, index: usize) -> Result<core::Camera, Box<dyn Error>> {
        Ok(core::Camera::new_id(
            self.id.get_id(index),
            self.lower_left_corner.to_vector3()?,
            self.horizontal.to_vector3()?,
            self.vertical.to_vector3()?,
            self.origin.to_vector3()?,
            self.lense_radius.to_value()?,
            self.time_from.to_value()?..self.time_to.to_value()?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CameraVerticalField {
    pub id: IdConstructor,
    pub v_fov: Value,
    pub aspect: Value,
}

impl CameraVerticalField {
    pub fn to_camera(&self, index: usize) -> Result<core::Camera, Box<dyn Error>> {
        Ok(core::Camera::from_vertical_field_id(
            self.id.get_id(index),
            self.v_fov.to_value()?,
            self.aspect.to_value()?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CameraLookAt {
    pub id: IdConstructor,
    pub look_from: Value,
    pub look_at: Value,
    pub vup: Value,
    pub v_fov: Value,
    pub aspect: Value,
    pub adepture: Value,
    pub focus_dist: Value,
    pub time_from: Value,
    pub time_to: Value,
}

impl CameraLookAt {
    pub fn to_camera(&self, index: usize) -> Result<core::Camera, Box<dyn Error>> {
        Ok(core::Camera::from_look_at_id(
            self.id.get_id(index),
            self.look_from.to_vector3()?,
            self.look_at.to_vector3()?,
            self.vup.to_vector3()?,
            self.v_fov.to_value()?,
            self.aspect.to_value()?,
            self.adepture.to_value()?,
            self.focus_dist.to_value()?,
            self.time_from.to_value()?..self.time_to.to_value()?,
        ))
    }
}

#[cfg(test)]
mod camera_test {
    use super::*;
    use ray_tracing_core::test;
    use ray_tracing_core::types::Vector3;

    #[test]
    fn camera_form_camera() {
        let c = core::Camera::new(
            Vector3::new(-2.0, -1.0, -1.0),
            Vector3::new(4.0, 0.0, 0.0),
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.1,
            0.0..1.0,
        );
        let ca = Camera::from_camera(&c).unwrap();
        assert_eq!(ca.lower_left_corner, Value::Vector3((-2.0, -1.0, -1.0)));
        assert_eq!(ca.horizontal, Value::Vector3((4.0, 0.0, 0.0)));
        assert_eq!(ca.vertical, Value::Vector3((0.0, 2.0, 0.0)));
        assert_eq!(ca.origin, Value::Vector3((0.0, 0.0, 0.0)));
        assert_eq!(ca.lense_radius, Value::Scalar(0.1));
        assert_eq!(ca.time_from, Value::Scalar(0.0));
        assert_eq!(ca.time_to, Value::Scalar(1.0));
    }

    #[test]
    fn camera_to_camera() {
        let ca = Camera {
            id: IdConstructor::Single(0),
            lower_left_corner: Value::Vector3((-2.0, -1.0, -1.0)),
            horizontal: Value::Vector3((4.0, 0.0, 0.0)),
            vertical: Value::Vector3((0.0, 2.0, 0.0)),
            origin: Value::Vector3((0.0, 0.0, 0.0)),
            lense_radius: Value::from_value(0.1).unwrap(),
            time_from: Value::from_value(0.0).unwrap(),
            time_to: Value::from_value(1.0).unwrap(),
        };
        let c = ca.to_camera(0).unwrap();
        test::assert_eq_vector3(&c.lower_left_corner, &Vector3::new(-2.0, -1.0, -1.0), 0.001);
        test::assert_eq_vector3(&c.horizontal, &Vector3::new(4.0, 0.0, 0.0), 0.001);
        test::assert_eq_vector3(&c.vertical, &Vector3::new(0.0, 2.0, 0.0), 0.001);
        test::assert_eq_vector3(&c.origin, &Vector3::new(0.0, 0.0, 0.0), 0.001);
        test::assert_eq_float(c.lense_radius, 0.1, 0.001);
        test::assert_eq_float(c.time.start, 0.0, 0.001);
        test::assert_eq_float(c.time.end, 1.0, 0.001);
    }

    // TODO test CameraVerticalField
    // TODO test CameraLookAt
}
