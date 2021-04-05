use self::core::*;
use self::environment::Sky;
use self::hit_able::collection::Collection;
use self::hit_able::instancing::*;
use self::hit_able::shape::*;
use self::hit_able::volume::*;
use self::material::*;
use self::texture::*;
use ray_tracing_core::random;
use ray_tracing_core::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::ops::Range;

/// Serialization of ray tracing textures
pub mod texture;

/// Serialization of ray tracing materials
pub mod material;

/// Serialization of ray tracing hit ables
pub mod hit_able;

/// Serialization of environment like sky
pub mod environment;

/// Serialization core objects  
pub mod core;

/// Create JSON from serialization
pub mod json;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdConstructor {
    Single(usize),
    Range((usize, usize)),
}

impl IdConstructor {
    pub fn get_id(&self, index: usize) -> usize {
        match self {
            IdConstructor::Single(id) => *id,
            IdConstructor::Range((start, _)) => *start + index,
        }
    }

    pub fn get_range(&self) -> Range<usize> {
        match self {
            IdConstructor::Single(id) => *id..(*id + 1),
            IdConstructor::Range((start, end)) => *start..*end,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            IdConstructor::Single(_) => 1,
            IdConstructor::Range((start, end)) => *end - *start,
        }
    }
}

impl From<usize> for IdConstructor {
    fn from(id: usize) -> Self {
        IdConstructor::Single(id)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdReference {
    Single(usize),
    Range((usize, usize)),
}

impl IdReference {
    pub fn get_id(&self, index: usize) -> usize {
        match self {
            IdReference::Single(id) => *id,
            IdReference::Range((start, _)) => *start + index,
        }
    }

    pub fn get_range(&self) -> Range<usize> {
        match self {
            IdReference::Single(id) => *id..(*id + 1),
            IdReference::Range((start, end)) => *start..*end,
        }
    }

    pub fn get_list(l: &Vec<IdReference>) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::default();
        for id_ref in l.iter() {
            ids.append(&mut id_ref.get_range().collect());
        }
        ids
    }

    pub fn get_id_from_list(l: Vec<IdReference>, index: usize) -> usize {
        IdReference::get_list(&l)[index]
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TaggedValue {
    Scalar(FSize),
    Range((FSize, FSize)),
    RandomScalar((FSize, FSize)),
    Vector3((FSize, FSize, FSize)),
    RandomVector3(((FSize, FSize), (FSize, FSize), (FSize, FSize))),
    Vector4((FSize, FSize, FSize, FSize)),
    RandomVector4(
        (
            (FSize, FSize),
            (FSize, FSize),
            (FSize, FSize),
            (FSize, FSize),
        ),
    ),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Scalar(FSize),
    Range((FSize, FSize)),
    Vector3((FSize, FSize, FSize)),
    Vector4((FSize, FSize, FSize, FSize)),
    Tagged(TaggedValue),
}

impl From<f64> for Value {
    fn from(s: f64) -> Self {
        Value::Scalar(s)
    }
}

impl TryFrom<Vec<f64>> for Value {
    type Error = &'static str;

    fn try_from(v: Vec<f64>) -> Result<Self, Self::Error> {
        if v.len() == 3 {
            Ok(Value::Vector3((v[0], v[1], v[2])))
        } else {
            Err("Vector has to have 3 scalars")
        }
    }
}

impl Value {
    pub fn from_value(value: FSize) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Scalar(value))
    }

    pub fn from_range(range: Range<FSize>) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Range((range.start, range.end)))
    }

    pub fn from_vector3(v: Vector3) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Vector3((v[0], v[1], v[2])))
    }

    pub fn from_point3(p: Point3) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Vector3((p[0], p[1], p[2])))
    }

    pub fn from_color_rgb(c: ColorRGB) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Vector3((c[0], c[1], c[2])))
    }

    pub fn from_color_rgba(c: ColorRGBA) -> Result<Value, Box<dyn Error>> {
        Ok(Value::Vector4((c[0], c[1], c[2], c[3])))
    }

    pub fn to_value(&self) -> Result<FSize, Box<dyn Error>> {
        match self {
            Value::Scalar(value) => Ok(*value),
            Value::Tagged(TaggedValue::Scalar(value)) => Ok(*value),
            Value::Tagged(TaggedValue::RandomScalar(range)) => Ok(Value::random_value(&range)),
            _ => Err("unexpected scalar type".into()),
        }
    }

    pub fn to_range(&self) -> Result<Range<FSize>, Box<dyn Error>> {
        match self {
            Value::Scalar(value) => Ok(*value..*value),
            Value::Tagged(TaggedValue::Scalar(value)) => Ok(*value..*value),
            Value::Tagged(TaggedValue::RandomScalar(range)) => {
                let value = Value::random_value(&range);
                Ok(value..value)
            }
            Value::Range(r) => Ok(r.0..r.1),
            Value::Tagged(TaggedValue::Range(r)) => Ok(r.0..r.1),
            _ => Err("unexpected scalar type".into()),
        }
    }

    pub fn to_vector3(&self) -> Result<Vector3, Box<dyn Error>> {
        match self {
            Value::Scalar(value) => Ok(Vector3::new(*value, *value, *value)),
            Value::Tagged(TaggedValue::Scalar(value)) => Ok(Vector3::new(*value, *value, *value)),
            Value::Tagged(TaggedValue::RandomScalar(range)) => {
                let v = Value::random_value(&range);
                Ok(Vector3::new(v, v, v))
            }
            Value::Vector3(v) => Ok(Vector3::new(v.0, v.1, v.2)),
            Value::Tagged(TaggedValue::Vector3(v)) => Ok(Vector3::new(v.0, v.1, v.2)),
            Value::Tagged(TaggedValue::RandomVector3(ranges)) => Ok(Vector3::new(
                Value::random_value(&ranges.0),
                Value::random_value(&ranges.1),
                Value::random_value(&ranges.2),
            )),
            _ => Err("unexpected scalar type".into()),
        }
    }

    pub fn to_vector4(&self) -> Result<Vector4, Box<dyn Error>> {
        match self {
            Value::Scalar(value) => Ok(Vector4::new(*value, *value, *value, 1.0)),
            Value::Tagged(TaggedValue::Scalar(value)) => {
                Ok(Vector4::new(*value, *value, *value, 1.0))
            }
            Value::Tagged(TaggedValue::RandomScalar(range)) => {
                let v = Value::random_value(&range);
                Ok(Vector4::new(v, v, v, 1.0))
            }
            Value::Vector3(v) => Ok(Vector4::new(v.0, v.1, v.2, 1.0)),
            Value::Tagged(TaggedValue::Vector3(v)) => Ok(Vector4::new(v.0, v.1, v.2, 1.0)),
            Value::Tagged(TaggedValue::RandomVector3(ranges)) => Ok(Vector4::new(
                Value::random_value(&ranges.0),
                Value::random_value(&ranges.1),
                Value::random_value(&ranges.2),
                1.0,
            )),
            Value::Vector4(v) => Ok(Vector4::new(v.0, v.1, v.2, v.3)),
            Value::Tagged(TaggedValue::Vector4(v)) => Ok(Vector4::new(v.0, v.1, v.2, v.3)),
            Value::Tagged(TaggedValue::RandomVector4(ranges)) => Ok(Vector4::new(
                Value::random_value(&ranges.0),
                Value::random_value(&ranges.1),
                Value::random_value(&ranges.2),
                1.0,
            )),
            _ => Err("unexpected scalar type".into()),
        }
    }

    pub fn to_point3(&self) -> Result<Point3, Box<dyn Error>> {
        Value::to_vector3(&self)
    }

    pub fn to_color_rgb(&self) -> Result<ColorRGB, Box<dyn Error>> {
        Value::to_vector3(&self)
    }

    pub fn to_color_rgba(&self) -> Result<ColorRGBA, Box<dyn Error>> {
        Value::to_vector4(&self)
    }

    fn random_value(range: &(FSize, FSize)) -> FSize {
        if range.0 < range.1 {
            random::generate_range(range.0..range.1)
        } else if range.1 < range.0 {
            random::generate_range(range.1..range.0)
        } else {
            range.0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RayTracingObject {
    None,
    Configuration(Configuration),
    Camera(Camera),
    CameraVerticalField(CameraVerticalField),
    CameraLookAt(CameraLookAt),
    Sky(Sky),
    ConstantTexture(ConstantTexture),
    BitmapFile(BitmapFile),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
    ColorFilter(ColorFilter),
    NoMaterial(NoMaterial),
    MaterialBlend(MaterialBlend),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
    Metal(Metal),
    Lambertian(Lambertian),
    Collection(Collection),
    Sphere(Sphere),
    MovableSphere(MovableSphere),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
    Cuboid(Cuboid),
    FlipNormals(FlipNormals),
    RotateX(RotateX),
    RotateY(RotateY),
    RotateZ(RotateZ),
    Translate(Translate),
    ConstantMedium(ConstantMedium),
}

pub struct Serializer {
    pub object_map: HashMap<String, RayTracingObject>,
}

#[cfg(test)]
mod serialization_test {
    use super::*;
    use ray_tracing_core::test;

    #[test]
    fn scalar_from_value_test() {
        assert_eq!(Value::from_value(1.0).unwrap(), Value::Scalar(1.0));
    }

    #[test]
    fn vector_of_scalars_from_vector3() {
        let v = Value::from_vector3(Vector3::new(1.0, 2.0, 3.0)).unwrap();
        match v {
            Value::Vector3(v) => {
                assert_eq!(v.0, 1.0);
                assert_eq!(v.1, 2.0);
                assert_eq!(v.2, 3.0);
            }
            _ => panic!("unexpected scalar type"),
        }
    }

    #[test]
    fn vector_of_scalars_from_point3() {
        let p = Value::from_point3(Vector3::new(-1.0, -2.0, -3.0)).unwrap();
        match p {
            Value::Vector3(p) => {
                assert_eq!(p.0, -1.0);
                assert_eq!(p.1, -2.0);
                assert_eq!(p.2, -3.0);
            }
            _ => panic!("unexpected scalar type"),
        }
    }

    #[test]
    fn vector_of_scalars_from_color_rgb() {
        let c = Value::from_point3(ColorRGB::new(0.0, 0.5, 1.0)).unwrap();
        match c {
            Value::Vector3(c) => {
                assert_eq!(c.0, 0.0);
                assert_eq!(c.1, 0.5);
                assert_eq!(c.2, 1.0);
            }
            _ => panic!("unexpected scalar type"),
        }
    }

    #[test]
    fn vector_of_scalars_from_color_rgba() {
        let c = Value::from_color_rgba(ColorRGBA::new(0.0, 0.5, 1.0, 1.0)).unwrap();
        match c {
            Value::Vector4(c) => {
                assert_eq!(c.0, 0.0);
                assert_eq!(c.1, 0.5);
                assert_eq!(c.2, 1.0);
                assert_eq!(c.3, 1.0);
            }
            _ => panic!("unexpected scalar type"),
        }
    }

    #[test]
    fn scalar_value_to_value_test() {
        test::assert_eq_float(Value::Scalar(1.0).to_value().unwrap(), 1.0, 0.001);
    }

    #[test]
    fn scalar_random_range_to_value_test() {
        test::assert_in_range(
            Value::Tagged(TaggedValue::RandomScalar((-1.0, 1.0)))
                .to_value()
                .unwrap(),
            -1.0..1.0,
        );
    }

    #[test]
    fn scalar_random_vector3_to_vector3_test() {
        test::assert_in_range_vector3(
            Value::Tagged(TaggedValue::RandomVector3((
                (-1.0, 1.0),
                (3.0, 2.0),
                (5.0, 5.0),
            )))
            .to_vector3()
            .unwrap(),
            Vector3::new(-1.0, 2.0, 4.99)..Vector3::new(1.0, 3.0, 5.01),
        );
    }

    #[test]
    fn vector_of_scalars_to_vector3() {
        let v = Value::Vector3((1.0, 2.0, 3.0));
        test::assert_eq_vector3(
            &v.to_vector3().unwrap(),
            &Vector3::new(1.0, 2.0, 3.0),
            0.001,
        );
    }

    #[test]
    fn vector_of_scalars_to_point3() {
        let p = Value::Vector3((-1.0, -2.0, -3.0));
        test::assert_eq_vector3(
            &p.to_point3().unwrap(),
            &Point3::new(-1.0, -2.0, -3.0),
            0.001,
        );
    }

    #[test]
    fn vector_of_scalars_to_color_rgb() {
        let c = Value::Vector3((0.0, 1.0, 0.5));
        test::assert_eq_vector3(
            &c.to_color_rgb().unwrap(),
            &ColorRGB::new(0.0, 1.0, 0.5),
            0.001,
        );
    }

    #[test]
    fn vector_of_scalars_to_color_rgba() {
        let c = Value::Vector4((0.0, 1.0, 0.5, 1.0));
        test::assert_eq_vector4(
            &c.to_color_rgba().unwrap(),
            &ColorRGBA::new(0.0, 1.0, 0.5, 1.0),
            0.001,
        );
    }
}
