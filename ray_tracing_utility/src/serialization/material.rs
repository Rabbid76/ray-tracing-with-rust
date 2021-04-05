use crate::serialization::texture::SerializeTexture;
use crate::serialization::RayTracingObject;
use ray_tracing_core::material;
use ray_tracing_core::texture;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

mod material_blend;
pub use self::material_blend::*;

mod dielectric;
pub use self::dielectric::*;

mod diffuse_light;
pub use self::diffuse_light::*;

mod isotropic;
pub use self::isotropic::*;

mod metal;
pub use self::metal::*;

mod no_material;
pub use self::no_material::*;

mod lambertian;
pub use self::lambertian::*;

pub struct SerializeMaterial {
    pub object_map: Rc<RefCell<HashMap<usize, RayTracingObject>>>,
}

impl SerializeMaterial {
    fn add_texture(&mut self, t: Arc<dyn texture::Texture>) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&t.get_id()) {
            t.accept(&mut SerializeTexture {
                object_map: self.object_map.clone(),
            })?;
        }
        Ok(())
    }

    fn add_material(&mut self, m: Arc<dyn material::Material>) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&m.get_id()) {
            m.accept(self)?;
        }
        Ok(())
    }
}

impl material::Visitor for SerializeMaterial {
    fn visit_no_material(&mut self, m: &material::NoMaterial) -> Result<(), Box<dyn Error>> {
        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::NoMaterial(NoMaterial::from_material(m)?),
        );
        Ok(())
    }

    fn visit_lambertian(&mut self, m: &material::Lambertian) -> Result<(), Box<dyn Error>> {
        self.add_texture(m.albedo.clone())?;

        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::Lambertian(Lambertian::from_material(m)?),
        );
        Ok(())
    }

    fn visit_metal(&mut self, m: &material::Metal) -> Result<(), Box<dyn Error>> {
        self.add_texture(m.albedo.clone())?;

        self.object_map
            .borrow_mut()
            .insert(m.id, RayTracingObject::Metal(Metal::from_material(m)?));
        Ok(())
    }

    fn visit_dielectric(&mut self, m: &material::Dielectric) -> Result<(), Box<dyn Error>> {
        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::Dielectric(Dielectric::from_material(m)?),
        );
        Ok(())
    }

    fn visit_isotropic(&mut self, m: &material::Isotropic) -> Result<(), Box<dyn Error>> {
        self.add_texture(m.albedo.clone())?;

        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::Isotropic(Isotropic::from_material(m)?),
        );
        Ok(())
    }

    fn visit_diffuse_light(&mut self, m: &material::DiffuseLight) -> Result<(), Box<dyn Error>> {
        self.add_texture(m.emit.clone())?;

        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::DiffuseLight(DiffuseLight::from_material(m)?),
        );
        Ok(())
    }

    fn visit_material_blend(&mut self, m: &material::MaterialBlend) -> Result<(), Box<dyn Error>> {
        for (_, material) in m.materials.iter() {
            self.add_material(material.clone())?;
        }

        self.object_map.borrow_mut().insert(
            m.id,
            RayTracingObject::MaterialBlend(MaterialBlend::from_material(m)?),
        );
        Ok(())
    }
}

#[cfg(test)]
mod serialize_material_test {
    use super::*;
    use ray_tracing_core::material::Material;
    use ray_tracing_core::texture::ConstantTexture;
    use ray_tracing_core::types::ColorRGBA;

    #[test]
    fn visit_no_material_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let nm = material::NoMaterial::new();
        nm.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 1);
        match &s.object_map.borrow_mut()[&nm.id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_lambertian_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct_id = ct.clone().id;
        let lm = material::Lambertian::new(ct);
        lm.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 2);
        match &s.object_map.borrow_mut()[&ct_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&lm.id] {
            RayTracingObject::Lambertian(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_metal_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct_id = ct.clone().id;
        let mm = material::Metal::new(0.5, ct);
        mm.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 2);
        match &s.object_map.borrow_mut()[&ct_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&mm.id] {
            RayTracingObject::Metal(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_dielectric_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let dm = material::Dielectric::new(
            0.5..0.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        );
        dm.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 1);
        match &s.object_map.borrow_mut()[&dm.id] {
            RayTracingObject::Dielectric(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_isotropic_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct_id = ct.clone().id;
        let im = material::Isotropic::new(ct);
        im.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 2);
        match &s.object_map.borrow_mut()[&ct_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&im.id] {
            RayTracingObject::Isotropic(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_diffuse_light_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct_id = ct.clone().id;
        let dm = material::DiffuseLight::new(ct);
        dm.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 2);
        match &s.object_map.borrow_mut()[&ct_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&dm.id] {
            RayTracingObject::DiffuseLight(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_material_blend_test() {
        let mut s = SerializeMaterial {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let nm1 = material::NoMaterial::new();
        let nm1_id = nm1.id;
        let nm2 = material::NoMaterial::new();
        let nm2_id = nm2.id;
        let mb = material::MaterialBlend::new(vec![(2.0, Arc::new(nm1)), (1.0, Arc::new(nm2))]);
        let mb_id = mb.id;
        mb.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 3);
        match &s.object_map.borrow_mut()[&nm1_id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&nm2_id] {
            RayTracingObject::NoMaterial(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&mb_id] {
            RayTracingObject::MaterialBlend(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }
}
