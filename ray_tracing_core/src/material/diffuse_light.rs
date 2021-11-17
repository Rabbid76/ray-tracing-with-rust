use crate::core::object::Object;
use crate::core::HitRecord;
use crate::core::ScatterRecord;
use crate::material::{Material, Visitor};
use crate::math::Ray;
use crate::texture::Texture;
use crate::types::{ColorRGB, ColorRGBA, FSize, Point3, TextureCoordinate};
use std::error::Error;
use std::sync::Arc;

pub struct DiffuseLight {
    pub id: usize,
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight {
            id: Object::new_id(),
            emit,
        }
    }
}

impl Material for DiffuseLight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn color_channels(&self, _: &TextureCoordinate, _: &Point3) -> ColorRGBA {
        ColorRGBA::new(0.0, 0.0, 0.0, 1.0)
    }

    fn scatter(
        &self,
        _self_material: Arc<dyn Material>,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
    ) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _: &Ray, _: &HitRecord, _: &Ray) -> FSize {
        1.0
    }

    fn has_alpha(&self) -> bool {
        false
    }

    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord) -> ColorRGB {
        if glm::dot(ray_in.direction, hit_record.normal) < 0.0 {
            self.emit
                .value(&hit_record.uv, &hit_record.position)
                .truncate(3)
        } else {
            ColorRGB::new(0.0, 0.0, 0.0)
        }
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_diffuse_light(&self)
    }
}

#[cfg(test)]
mod diffuse_light_test {
    use super::*;
    use crate::material::NoMaterial;
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::{ColorRGBA, Point3, TextureCoordinate, Vector3};

    #[test]
    fn scatter_test() {
        let m = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(1.0, 0.0, 0.0, 1.0),
        ))));
        let result = m.scatter(
            m.clone(),
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::new(
                0.0,
                TextureCoordinate::from_uv(0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Arc::new(NoMaterial::new()),
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            ),
        );
        match result {
            Some(_) => panic!("no result"),
            None => (),
        }
    }

    #[test]
    fn emitted_test() {
        let m = DiffuseLight::new(Arc::new(ConstantTexture::new(ColorRGBA::new(
            1.0, 0.0, 0.0, 1.0,
        ))));
        let c = m.emitted(
            &Ray::new_ray(Point3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)),
            &HitRecord::new(
                0.0,
                TextureCoordinate::from_uv(0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Arc::new(NoMaterial::new()),
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            ),
        );
        test::assert_eq_vector3(&c, &ColorRGB::new(1.0, 0.0, 0.0), 0.01);
    }
}
