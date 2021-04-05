use crate::types::{ColorRGB, FSize, Point3, TextureCoordinate, Vector3};
use core::f64::consts::PI;
use rand::Rng;
use std::ops::Range;

/// Generate random axis
pub fn generate_axis() -> usize {
    rand::thread_rng().gen_range(0..3)
}

/// Generate random axis
pub fn generate_from_range(range: Range<usize>) -> usize {
    rand::thread_rng().gen_range(range)
}

/// Generate a single floating point value in the range [0.0, 1.0]
pub fn generate_size() -> FSize {
    let value: FSize = rand::thread_rng().gen();
    value
}

/// Generate a single floating point value in a specific range
pub fn generate_range(range: Range<FSize>) -> FSize {
    let value: FSize = rand::thread_rng().gen_range(range);
    value
}

/// Generate a floating point tuple in a specific range
pub fn generate_range2d(range: &Range<(FSize, FSize)>) -> (FSize, FSize) {
    (
        generate_range(range.start.0..range.end.0),
        generate_range(range.start.1..range.end.1),
    )
}

/// Generate a single floating point value in range [-1.0, 1.0]
pub fn generate_unit() -> FSize {
    let value: FSize = rand::thread_rng().gen();
    value * 2.0 - 1.0
}

/// Generate a single floating point value in range [0.0, 1.0]
pub fn generate_unit_abs() -> FSize {
    let value: FSize = rand::thread_rng().gen();
    value
}

/// Generate a random vector, whose length is less than or equal 1.0
pub fn generate_unit_sphere() -> Vector3 {
    loop {
        let v = generate_vector3();
        if glm::dot(v, v) < 1.0 {
            break v;
        }
    }
}

/// Generate a random vector
pub fn generate_cosine_direction() -> Vector3 {
    let r1 = generate_size();
    let r2 = generate_size();
    let z = FSize::sqrt(1.0 - r2);
    let phi = 2.0 * PI * r1;
    let x = FSize::cos(phi) * 2.0 * FSize::sqrt(r2);
    let y = FSize::sin(phi) * 2.0 * FSize::sqrt(r2);
    Vector3::new(x, y, z)
}

/// Generate a random vector
pub fn generate_to_sphere(radius: FSize, distance_squared: FSize) -> Vector3 {
    let r1 = generate_size();
    let r2 = generate_size();
    let z = 1.0 + r2 * (FSize::sqrt(1.0 - radius * radius / distance_squared) - 1.0);
    let phi = 2.0 * PI * r1;
    let x = FSize::cos(phi) * FSize::sqrt(1.0 - z * z);
    let y = FSize::sin(phi) * FSize::sqrt(1.0 - z * z);
    Vector3::new(x, y, z)
}

/// Generate a random RGB color
pub fn generate_rgb() -> ColorRGB {
    ColorRGB::new(
        generate_unit_abs(),
        generate_unit_abs(),
        generate_unit_abs(),
    )
}

/// Generate a random vector
pub fn generate_vector3() -> Vector3 {
    Vector3::new(generate_unit(), generate_unit(), generate_unit())
}

/// Generate a random point
pub fn generate_point3() -> Point3 {
    Point3::new(generate_unit(), generate_unit(), generate_unit())
}

/// Generate a texture coordinate
pub fn generate_uv() -> TextureCoordinate {
    TextureCoordinate::from_uv(generate_unit_abs(), generate_unit_abs())
}

#[cfg(test)]
mod random_test {
    use super::*;
    use crate::test;

    #[test]
    fn generate_size_test() {
        let value = generate_size();
        test::assert_in_range(value, 0.0..1.0);
    }

    #[test]
    fn generate_range_test() {
        let value = generate_range(-1.0..1.0);
        test::assert_in_range(value, -1.0..1.0);
    }

    #[test]
    fn generate_unit_test() {
        let value = generate_unit();
        test::assert_in_range(value, -1.0..1.0);
    }

    #[test]
    fn generate_unit_abs_test() {
        let value = generate_unit_abs();
        test::assert_in_range(value, 0.0..1.0);
    }

    #[test]
    fn generate_unit_sphere_test() {
        let v = generate_unit_sphere();
        for i in 0..3 {
            test::assert_in_range(v[i], -1.0..1.0);
        }
        assert!(glm::length(v) <= 1.0);
    }

    #[test]
    fn generate_rgb_test() {
        let c = generate_rgb();
        for i in 0..3 {
            test::assert_in_range(c[i], 0.0..1.0);
        }
    }

    #[test]
    fn generate_vector_test() {
        let v = generate_vector3();
        for i in 0..3 {
            test::assert_in_range(v[i], -1.0..1.0);
        }
    }

    #[test]
    fn generate_point_test() {
        let p = generate_point3();
        for i in 0..3 {
            test::assert_in_range(p[i], -1.0..1.0);
        }
    }

    #[test]
    fn generate_uv_test() {
        let uv = generate_uv();
        test::assert_in_range(uv.u, 0.0..1.0);
        test::assert_in_range(uv.v, 0.0..1.0);
    }
}
