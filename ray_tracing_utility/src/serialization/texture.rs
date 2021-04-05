use crate::serialization::RayTracingObject;
use ray_tracing_core::texture;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

mod constant_texture;
pub use self::constant_texture::*;

mod bitmap_file;
pub use self::bitmap_file::*;

mod checker_texture;
pub use self::checker_texture::*;

mod noise_texture;
pub use self::noise_texture::*;

mod color_filter;
pub use self::color_filter::*;

pub struct SerializeTexture {
    pub object_map: Rc<RefCell<HashMap<usize, RayTracingObject>>>,
}

impl SerializeTexture {
    fn add_texture(&mut self, t: Arc<dyn texture::Texture>) -> Result<(), Box<dyn Error>> {
        if !self.object_map.borrow().contains_key(&t.get_id()) {
            t.accept(self)?;
        }
        Ok(())
    }
}

impl texture::Visitor for SerializeTexture {
    fn visit_constant_texture(
        &mut self,
        t: &texture::ConstantTexture,
    ) -> Result<(), Box<dyn Error>> {
        self.object_map.borrow_mut().insert(
            t.id,
            RayTracingObject::ConstantTexture(ConstantTexture::from_texture(t)?),
        );
        Ok(())
    }

    fn visit_bitmap_texture(&mut self, _: &texture::BitmapTexture) -> Result<(), Box<dyn Error>> {
        Err("not yet implemented".into())
    }

    fn visit_checker_texture(&mut self, t: &texture::CheckerTexture) -> Result<(), Box<dyn Error>> {
        self.add_texture(t.even_texture.clone())?;
        self.add_texture(t.odd_texture.clone())?;

        self.object_map.borrow_mut().insert(
            t.id,
            RayTracingObject::CheckerTexture(CheckerTexture::from_texture(t)?),
        );
        Ok(())
    }

    fn visit_noise_texture(&mut self, t: &texture::NoiseTexture) -> Result<(), Box<dyn Error>> {
        self.add_texture(t.min_texture.clone())?;
        self.add_texture(t.max_texture.clone())?;

        self.object_map.borrow_mut().insert(
            t.id,
            RayTracingObject::NoiseTexture(NoiseTexture::from_texture(t)?),
        );
        Ok(())
    }

    fn visit_color_filter(&mut self, t: &texture::ColorFilter) -> Result<(), Box<dyn Error>> {
        self.add_texture(t.texture.clone())?;

        self.object_map.borrow_mut().insert(
            t.id,
            RayTracingObject::ColorFilter(ColorFilter::from_texture(t)?),
        );
        Ok(())
    }
}

#[cfg(test)]
mod serialize_texture_test {
    use super::*;
    use ray_tracing_core::texture::Texture;
    use ray_tracing_core::types::{ColorRGBA, Vector3};

    #[test]
    fn visit_constant_texture_test() {
        let mut s = SerializeTexture {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = texture::ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0));
        ct.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 1);
        match &s.object_map.borrow_mut()[&ct.id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_checker_texture_test() {
        let mut s = SerializeTexture {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct1 = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct1_id = ct1.clone().id;
        let ct2 = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            1.0, 1.0, 1.0, 1.0,
        )));
        let ct2_id = ct2.clone().id;
        let ct = texture::CheckerTexture::new(Vector3::new(1.0, 1.0, 1.0), ct1, ct2);
        ct.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 3);
        match &s.object_map.borrow_mut()[&ct1_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&ct2_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&ct.id] {
            RayTracingObject::CheckerTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_noise_texture_test() {
        let mut s = SerializeTexture {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct1 = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            0.0, 0.0, 0.0, 1.0,
        )));
        let ct1_id = ct1.clone().id;
        let ct2 = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            1.0, 1.0, 1.0, 1.0,
        )));
        let ct2_id = ct2.clone().id;
        let ct = texture::NoiseTexture::new(1.0, texture::NoiseType::Default, ct1, ct2);
        ct.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 3);
        match &s.object_map.borrow_mut()[&ct1_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&ct2_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&ct.id] {
            RayTracingObject::NoiseTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }

    #[test]
    fn visit_color_filter_test() {
        let mut s = SerializeTexture {
            object_map: Rc::new(RefCell::new(HashMap::default())),
        };
        let ct = Arc::new(texture::ConstantTexture::new(ColorRGBA::new(
            1.0, 1.0, 1.0, 1.0,
        )));
        let ct_id = ct.clone().id;
        let cf = texture::ColorFilter::new(
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            ColorRGBA::new(1.0, 0.5, 0.25, 1.0),
            ColorRGBA::new(0.0, 0.0, 0.0, 0.0),
            ct,
        );
        cf.accept(&mut s).unwrap();
        assert_eq!(s.object_map.borrow_mut().len(), 2);
        match &s.object_map.borrow_mut()[&ct_id] {
            RayTracingObject::ConstantTexture(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
        match &s.object_map.borrow_mut()[&cf.id] {
            RayTracingObject::ColorFilter(_) => (),
            _ => panic!("unexpected ray tracing object"),
        };
    }
}
