use glm;
use std::f64::consts::PI;

/// Floating point data type for ray tracing calculations
pub type FSize = f64;

/// Floating point data type that represents a time
pub type Time = FSize;

/// Data type that represents an RGB color
pub type ColorRGB = glm::Vector3<FSize>;

/// Data type that represents an RGBA color
pub type ColorRGBA = glm::Vector4<FSize>;

/// Data type that represents a three-dimensional Cartesian point
pub type Point3 = glm::Vector3<FSize>;

/// Data type that represents a two-dimensional Cartesian vector
pub type Vector3 = glm::Vector3<FSize>;

/// Data type that represents a three-dimensional Cartesian vector
pub type Vector2 = glm::Vector2<FSize>;

/// Data type that represents a three-dimensional Cartesian vector
pub type Vector4 = glm::Vector4<FSize>;

/// Object that represents texture coordinates
///
/// We do not use `glm :: Vector2` here, as the texture coordinates may be extended by a `w` component or a layer in the future
pub struct TextureCoordinate {
    pub u: FSize,
    pub v: FSize,
}

impl TextureCoordinate {
    pub fn from_uv(u: FSize, v: FSize) -> TextureCoordinate {
        TextureCoordinate { u, v }
    }

    /// TODO lazy computation
    pub fn from_sphere(p: &Vector3) -> TextureCoordinate {
        let phi = FSize::atan2(p.z, p.x);
        let theta = FSize::asin(p.y);
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        TextureCoordinate { u, v }
    }
}

pub fn refract(v: &Vector3, n: &Vector3, ni_over_nt: FSize) -> Option<Vector3> {
    let uv = glm::normalize(*v);
    let dt = glm::dot(uv, *n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: FSize, ref_idx: FSize) -> FSize {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * FSize::powf(1.0 - cosine, 5.0)
}

pub fn distance_square(p1: Point3, p2: Point3) -> FSize {
    let d = p2 - p1;
    d.x * d.x + d.y * d.y + d.z * d.z
}

// TODO test
