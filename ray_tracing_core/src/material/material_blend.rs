use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::random;
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::sync::Arc;

/// Material node for blending materials
///
/// The materials are weighted and blended.
///
pub struct MaterialBlend {
    pub id: usize,
    pub materials: Vec<(FSize, Arc<dyn Material>)>,
    weights: Vec<FSize>,
}

impl MaterialBlend {
    pub fn new(materials: Vec<(FSize, Arc<dyn Material>)>) -> MaterialBlend {
        MaterialBlend::new_id(Object::new_id(), materials)
    }

    pub fn new_id(id: usize, materials: Vec<(FSize, Arc<dyn Material>)>) -> MaterialBlend {
        let weight_sum: FSize = materials.iter().map(|(w, _)| w).sum();
        let mut weights = Vec::<FSize>::default();
        let mut sum = 0.0;
        for i in 0..materials.len() {
            sum += materials[i].0;
            weights.push(sum / weight_sum);
        }
        MaterialBlend {
            id: id,
            materials,
            weights,
        }
    }
}

impl Material for MaterialBlend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn scatter(
        &self,
        _self_material: Arc<dyn Material>,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<ScatterRecord> {
        let random_weight = random::generate_size();
        let mut i = 0;
        while i < self.weights.len() - 1 && random_weight > self.weights[i] {
            i += 1;
        }
        self.materials[i]
            .1
            .scatter(self.materials[i].1.clone(), ray_in, hit_record)
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> FSize {
        1.0
    }

    fn has_alpha(&self) -> bool {
        self.materials.iter().any(|(_, m)| m.has_alpha())
    }

    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord) -> ColorRGB {
        let random_weight = random::generate_size();
        let mut i = 0;
        while i < self.weights.len() - 1 && random_weight > self.weights[i] {
            i += 1;
        }
        self.materials[i].1.emitted(ray_in, hit_record)
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_material_blend(&self)
    }
}

#[cfg(test)]
mod material_blend_test {
    use super::*;
    use crate::material::{DiffuseLight, Lambertian, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};

    #[test]
    fn scatter_test() {
        let m1 = Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let m2 = Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let m = Arc::new(MaterialBlend::new(vec![
            (1.0, Arc::new(m1)),
            (1.0, Arc::new(m2)),
        ]));
        let result = m.scatter(
            m.clone(),
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::new(
                0.0,
                TextureCoordinate::from_uv(0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Arc::new(NoMaterial::new()),
            ),
        );
        match result {
            Some(scatter_record) => {
                test::assert_eq_vector3(
                    &scatter_record.attenuation,
                    &ColorRGB::new(0.0, 0.0, 0.0),
                    0.001,
                );
                test::assert_eq_float(scatter_record.alpha, 1.0, 0.001);
                test::assert_eq_vector3(
                    &scatter_record.ray.origin,
                    &Point3::new(0.0, 0.0, 0.0),
                    0.001,
                );
                test::assert_in_range_vector3(
                    scatter_record.ray.direction,
                    Vector3::new(-1.0, -1.0, 0.0)..Vector3::new(1.0, 1.0, 2.0),
                );
                assert!(glm::length(scatter_record.ray.direction) <= 2.0);
                test::assert_eq_float(scatter_record.ray.time, 0.0, 0.001);
            }
            None => panic!("no result"),
        }
    }

    #[test]
    fn emitted_test() {
        let m1 = Lambertian::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 1.0, 0.0, 1.0,
        ))));
        let m2 = DiffuseLight::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        ))));
        let m = MaterialBlend::new(vec![(1.0, Arc::new(m1)), (1.0, Arc::new(m2))]);
        let c = m.emitted(
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::new(
                0.0,
                TextureCoordinate::from_uv(0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Arc::new(NoMaterial::new()),
            ),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(0.0, 0.0, 0.0), 0.001);
    }
}
