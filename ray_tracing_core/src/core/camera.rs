use crate::core::object::Object;
use crate::math::Ray;
use crate::random;
use crate::types::{FSize, Vector3};
use glm;
use std::f64::consts::PI;
use std::ops::Range;

/// Camera object
///
/// Represents a camera by position and field of view
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub id: usize,
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub origin: Vector3,
    pub _u: Vector3,
    pub _v: Vector3,
    pub _w: Vector3,
    pub lense_radius: FSize,
    pub time: Range<FSize>,
}

impl Camera {
    pub fn new_id(
        id: usize,
        lower_left_corner: Vector3,
        horizontal: Vector3,
        vertical: Vector3,
        origin: Vector3,
        lense_radius: FSize,
        time: Range<FSize>,
    ) -> Camera {
        Camera {
            id,
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            _u: glm::normalize(horizontal),
            _v: glm::normalize(vertical),
            _w: glm::normalize(origin - lower_left_corner - horizontal * 0.5 - vertical * 0.5),
            lense_radius,
            time,
        }
    }

    pub fn new(
        lower_left_corner: Vector3,
        horizontal: Vector3,
        vertical: Vector3,
        origin: Vector3,
        lense_radius: FSize,
        time: Range<FSize>,
    ) -> Camera {
        Camera::new_id(
            Object::new_id(),
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lense_radius,
            time,
        )
    }

    pub fn from_vertical_field(v_fov: FSize, aspect: FSize) -> Camera {
        Camera::from_vertical_field_id(Object::new_id(), v_fov, aspect)
    }

    pub fn from_vertical_field_id(id: usize, v_fov: FSize, aspect: FSize) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = FSize::tan(theta / 2.0);
        let half_width = half_height * aspect;
        Camera::new_id(
            id,
            Vector3::new(-half_width, -half_height, -1.0),
            Vector3::new(2.0 * half_width, 0.0, 0.0),
            Vector3::new(0.0, 2.0 * half_height, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.0,
            0.0..0.0,
        )
    }

    pub fn from_look_at(
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        v_fov: FSize,
        aspect: FSize,
        adepture: FSize,
        focus_dist: FSize,
        time: Range<FSize>,
    ) -> Camera {
        Camera::from_look_at_id(
            Object::new_id(),
            look_from,
            look_at,
            vup,
            v_fov,
            aspect,
            adepture,
            focus_dist,
            time,
        )
    }

    pub fn from_look_at_id(
        id: usize,
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        v_fov: FSize,
        aspect: FSize,
        adepture: FSize,
        focus_dist: FSize,
        time: Range<FSize>,
    ) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = FSize::tan(theta / 2.0);
        let half_width = half_height * aspect;
        let w = glm::normalize(look_from - look_at);
        let u = glm::cross(vup, w);
        let v = glm::cross(w, u);
        Camera::new_id(
            id,
            look_from - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
            u * half_width * 2.0 * focus_dist,
            v * half_height * 2.0 * focus_dist,
            look_from,
            adepture / 2.0,
            time,
        )
    }

    pub fn get(&self, u: FSize, v: FSize) -> Ray {
        let rd = random::generate_unit_sphere() * self.lense_radius;
        let offset = u * rd.x + v * rd.y;
        let time = if self.time.start == self.time.end {
            self.time.start
        } else {
            self.time.start
                + random::generate_range(self.time.start..self.time.end)
                    * (self.time.end - self.time.start)
        };
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
            time,
            None,
        )
    }

    pub fn change_aspect(&mut self, aspect: FSize) {
        let len_x = glm::length(self.horizontal);
        let len_y = glm::length(self.vertical);
        let new_len_x = len_y * aspect;
        let horizontal_dir = glm::normalize(self.horizontal);
        self.horizontal = horizontal_dir * new_len_x;
        self.lower_left_corner =
            self.lower_left_corner + horizontal_dir * (len_x - new_len_x) / 2.0;
    }
}

#[cfg(test)]
mod camera_test {
    use super::*;
    use crate::test;

    #[test]
    fn from_vertical_field_test() {
        let c = Camera::from_vertical_field(90.0, 1.0);
        let r = c.get(0.5, 0.5);
        test::assert_eq_vector3(&r.direction, &Vector3::new(0.0, 0.0, -1.0), 0.001);
    }

    #[test]
    fn from_look_at_test() {
        let c = Camera::from_look_at(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
            0.0,
            1.0,
            0.0..0.0,
        );
        let r = c.get(0.5, 0.5);
        test::assert_eq_vector3(&r.direction, &Vector3::new(0.8944, 0.0, 0.4472), 0.001);
    }

    #[test]
    fn ray_test() {
        let c = Camera::new(
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            0.0,
            0.0..0.0,
        );
        let r = c.get(0.5, 0.5);
        test::assert_eq_vector3(&r.direction, &Vector3::new(0.0, 0.0, 1.0), 0.001);
    }

    #[test]
    fn from_change_aspect_test() {
        let mut c = Camera::new(
            Vector3::new(-1.0, -0.5, 0.0),
            Vector3::new(2.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            0.0,
            0.0..0.0,
        );
        c.change_aspect(1.5);
        test::assert_eq_vector3(&c.horizontal, &Vector3::new(1.5, 0.0, 0.0), 0.001);
        test::assert_eq_vector3(&c.lower_left_corner, &Vector3::new(-0.75, -0.5, 0.0), 0.001);
    }
}
