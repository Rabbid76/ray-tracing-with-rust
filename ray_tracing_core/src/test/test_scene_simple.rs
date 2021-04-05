use crate::core::{Camera, Configuration, Scene};
use crate::environment::Sky;
use crate::hit_able::collection::BVHNode;
use crate::hit_able::shape::Sphere;
use crate::material::Lambertian;
use crate::texture::ConstantTexture;
use crate::types::{ColorRGB, ColorRGBA, Point3, Vector3};
use std::sync::Arc;

pub struct TestSceneSimple {
    pub scene: Scene,
}

impl TestSceneSimple {
    pub fn new() -> TestSceneSimple {
        let sphere_texture = Arc::new(ConstantTexture::new(ColorRGBA::new(0.5, 0.1, 0.1, 1.0)));
        let sphere_material = Arc::new(Lambertian::new(sphere_texture.clone()));
        let sphere = Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            sphere_material.clone(),
        ));

        let ground_texture = Arc::new(ConstantTexture::new(ColorRGBA::new(0.1, 0.1, 0.1, 1.0)));
        let ground_material = Arc::new(Lambertian::new(ground_texture.clone()));
        let ground = Arc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            ground_material.clone(),
        ));

        let world = BVHNode::new(&vec![ground.clone(), sphere.clone()], 0.0..0.0);

        let camera = Camera::new(
            Vector3::new(-2.0, -1.0, -1.0),
            Vector3::new(4.0, 0.0, 0.0),
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            0.0,
            0.0..0.0,
        );

        let sky = Sky::new(ColorRGB::new(1.0, 1.0, 1.0), ColorRGB::new(0.5, 0.7, 1.0));

        let scene = Scene::new(
            Configuration::default(),
            Arc::new(camera),
            Arc::new(sky),
            world,
            None,
        );

        TestSceneSimple { scene }
    }
}
