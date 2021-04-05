use crate::types::{FSize, Vector3, Vector4};
use std::cmp::PartialOrd;
use std::ops::Range;

/// floating point comparison
#[allow(dead_code)]
pub fn assert_eq_float(a: f64, b: f64, eps: f64) {
    assert!(
        f64::abs(a - b) < eps,
        "floating point values are not equal: {} != {}",
        a,
        b
    );
}

/// Vector3 floating point comparison
#[allow(dead_code)]
pub fn assert_eq_vector3(a: &Vector3, b: &Vector3, eps: FSize) {
    for i in 0..3 {
        assert_eq_float(a[i], b[i], eps);
    }
}

/// Vector3 floating point comparison
#[allow(dead_code)]
pub fn assert_eq_vector4(a: &Vector4, b: &Vector4, eps: FSize) {
    for i in 0..4 {
        assert_eq_float(a[i], b[i], eps);
    }
}

/// floating point value in range
#[allow(dead_code)]
pub fn assert_in_range<T>(value: T, range: Range<T>)
where
    T: PartialOrd,
{
    assert!(
        range.contains(&value),
        "floating point value is not in range"
    );
}

/// Vector3 in range
#[allow(dead_code)]
pub fn assert_in_range_vector3(v: Vector3, range: Range<Vector3>) {
    for i in 0..3 {
        assert_in_range(v[i], range.start[i]..range.end[i]);
    }
}

/// Vector3 in range
#[allow(dead_code)]
pub fn assert_in_range_vector4(v: Vector4, range: Range<Vector4>) {
    for i in 0..4 {
        assert_in_range(v[i], range.start[i]..range.end[i]);
    }
}

#[cfg(test)]
mod test_helper_test {
    use super::*;

    #[test]
    fn assert_eq_float_test() {
        assert_eq_float(1.0, 0.99, 0.1);
    }

    #[test]
    #[should_panic(expected = "floating point values are not equal")]
    fn assert_eq_float_test_fail() {
        assert_eq_float(1.0, 0.99, 0.001);
    }

    #[test]
    fn assert_equ_vector3_test() {
        assert_eq_vector3(
            &Vector3::new(1.0, 2.0, 3.0),
            &Vector3::new(0.99, 2.01, 3.0),
            0.1,
        );
    }

    #[test]
    #[should_panic(expected = "floating point values are not equal")]
    fn assert_eq_vector3_test_fail() {
        assert_eq_vector3(
            &Vector3::new(1.0, 2.0, 3.0),
            &Vector3::new(1.0, 1.99, 3.0),
            0.001,
        );
    }

    #[test]
    fn assert_in_range_test() {
        assert_in_range(0.5, 0.0..1.0);
    }

    #[test]
    #[should_panic(expected = "floating point value is not in range")]
    fn assert_in_range_test_fail() {
        assert_in_range(1.5, 0.0..1.0);
    }

    #[test]
    fn assert_in_range_vector3_test() {
        assert_in_range_vector3(
            Vector3::new(0.1, 0.5, 0.9),
            Vector3::new(0.0, 0.0, 0.0)..Vector3::new(1.0, 1.0, 1.0),
        );
    }

    #[test]
    #[should_panic(expected = "floating point value is not in range")]
    fn assert_in_range_vector3_test_fail() {
        assert_in_range_vector3(
            Vector3::new(0.1, 1.5, 0.9),
            Vector3::new(0.0, 0.0, 0.0)..Vector3::new(1.0, 1.0, 1.0),
        );
    }
}
