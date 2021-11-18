use crate::serialization::core::{Camera, Configuration};
use crate::serialization::environment::SerializeEnvironment;
use crate::serialization::geometry::SerializeGeometry;
use crate::serialization::{IdConstructor, IdReference, RayTracingObject};
use ray_tracing_core::core;
use ray_tracing_core::environment;
use ray_tracing_core::geometry;
use ray_tracing_core::material;
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    pub configuration_id: usize,

    pub camera_id: usize,

    pub sky_id: usize,

    pub root_node_id: usize,

    #[serde(default = "Scene::default_light_node")]
    pub light_node_id: usize,

    pub objects: Vec<RayTracingObject>,
}

pub struct DeserializeOptions {
    pub root_path: Option<String>,
}

impl DeserializeOptions {
    pub fn default() -> DeserializeOptions {
        DeserializeOptions { root_path: None }
    }

    pub fn form_path(root_path: &Path) -> DeserializeOptions {
        DeserializeOptions {
            root_path: match root_path.to_str() {
                Some(path) => Some(String::from(path)),
                None => None,
            },
        }
    }
}

impl Scene {
    fn default_light_node() -> usize {
        0
    }

    pub fn from_scene(s: &core::Scene) -> Result<Scene, Box<dyn Error>> {
        let object_map = Rc::new(RefCell::new(HashMap::<usize, RayTracingObject>::default()));
        object_map.borrow_mut().insert(
            s.configuration.id,
            RayTracingObject::Configuration(Configuration::from_configuration(&s.configuration)?),
        );
        object_map.borrow_mut().insert(
            s.camera.id,
            RayTracingObject::Camera(Camera::from_camera(&s.camera)?),
        );
        let mut se = SerializeEnvironment {
            object_map: object_map.clone(),
        };
        s.sky.accept(&mut se)?;
        let mut sh = SerializeGeometry {
            object_map: object_map.clone(),
            collection: None,
        };
        s.world.accept(&mut sh)?;
        drop(sh);
        //let objects = object_map.take().into_iter().map(|(_id, obj)| obj).collect();
        let mut tuples: Vec<(usize, RayTracingObject)> = object_map.take().into_iter().collect();
        tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let objects = tuples.iter().map(|(_id, obj)| obj.clone()).collect();
        Ok(Scene {
            camera_id: s.camera.id,
            root_node_id: s.world.get_id(),
            sky_id: s.sky.get_id(),
            configuration_id: s.configuration.id,
            light_node_id: 0,
            objects: objects,
        })
    }

    pub fn to_scene(&self) -> Result<core::Scene, Box<dyn Error>> {
        self.to_scene_with_options(&DeserializeOptions::default())
    }

    pub fn insert_texture<T, F>(
        container: &mut HashMap<usize, Arc<dyn texture::Texture>>,
        id: &IdConstructor,
        object: &T,
        convert: F,
    ) where
        F: Fn(&T, usize, &HashMap<usize, Arc<dyn texture::Texture>>) -> Arc<dyn texture::Texture>,
    {
        for index in 0..id.len() {
            container.insert(id.get_id(index), convert(object, index, container));
        }
    }

    pub fn insert_material<T, F>(
        container: &mut HashMap<usize, Arc<dyn material::Material>>,
        id: &IdConstructor,
        object: &T,
        convert: F,
    ) where
        F: Fn(
            &T,
            usize,
            &HashMap<usize, Arc<dyn material::Material>>,
        ) -> Arc<dyn material::Material>,
    {
        for index in 0..id.len() {
            container.insert(id.get_id(index), convert(object, index, container));
        }
    }

    pub fn insert_geometry<T, F>(
        container: &mut HashMap<usize, Arc<dyn geometry::Geometry>>,
        id: &IdConstructor,
        object: &T,
        convert: F,
    ) where
        F: Fn(
            &T,
            usize,
            &HashMap<usize, Arc<dyn geometry::Geometry>>,
        ) -> Arc<dyn geometry::Geometry>,
    {
        for index in 0..id.len() {
            container.insert(id.get_id(index), convert(object, index, container));
        }
    }

    pub fn to_scene_with_options(
        &self,
        deserialize_options: &DeserializeOptions,
    ) -> Result<core::Scene, Box<dyn Error>> {
        let mut configuration_map = HashMap::<usize, core::Configuration>::default();
        let mut camera_map = HashMap::<usize, Arc<core::Camera>>::default();
        let mut texture_map = HashMap::<usize, Arc<dyn texture::Texture>>::default();
        for obj in self.objects.iter() {
            match obj {
                RayTracingObject::Configuration(c) => {
                    for index in 0..c.id.len() {
                        configuration_map.insert(c.id.get_id(index), c.to_configuration(index)?);
                    }
                }
                RayTracingObject::Camera(c) => {
                    for index in 0..c.id.len() {
                        camera_map.insert(c.id.get_id(index), Arc::new(c.to_camera(index)?));
                    }
                }
                RayTracingObject::CameraVerticalField(c) => {
                    for index in 0..c.id.len() {
                        camera_map.insert(c.id.get_id(index), Arc::new(c.to_camera(index)?));
                    }
                }
                RayTracingObject::CameraLookAt(c) => {
                    for index in 0..c.id.len() {
                        camera_map.insert(c.id.get_id(index), Arc::new(c.to_camera(index)?));
                    }
                }
                RayTracingObject::ConstantTexture(t) => {
                    Scene::insert_texture(&mut texture_map, &t.id, t, |t, i, _| {
                        Arc::new(t.to_texture(i).unwrap())
                    })
                }
                RayTracingObject::BitmapFile(t) => {
                    Scene::insert_texture(&mut texture_map, &t.id, t, |t, i, _| {
                        Arc::new(t.to_texture(i, &deserialize_options.root_path).unwrap())
                    })
                }

                RayTracingObject::CheckerTexture(t) => {
                    Scene::insert_texture(&mut texture_map, &t.id, t, |t, i, tm| {
                        Arc::new(
                            t.to_texture(
                                i,
                                Scene::get_texture(tm, &t.even_texture, i),
                                Scene::get_texture(tm, &t.odd_texture, i),
                            )
                            .unwrap(),
                        )
                    })
                }
                RayTracingObject::BlendTexture(t) => {
                    Scene::insert_texture(&mut texture_map, &t.id, t, |t, i, tm| {
                        Arc::new(
                            t.to_texture(
                                i,
                                Scene::get_texture(tm, &t.first_texture, i),
                                Scene::get_texture(tm, &t.second_texture, i),
                                Scene::get_texture(tm, &t.mask_texture, i),
                            )
                            .unwrap(),
                        )
                    })
                }
                RayTracingObject::NoiseTexture(t) => {
                    Scene::insert_texture(&mut texture_map, &t.id, t, |t, i, tm| {
                        Arc::new(
                            t.to_texture(
                                i,
                                Scene::get_texture(tm, &t.min_texture, i),
                                Scene::get_texture(tm, &t.max_texture, i),
                            )
                            .unwrap(),
                        )
                    })
                }
                RayTracingObject::ColorFilter(f) => {
                    Scene::insert_texture(&mut texture_map, &f.id, f, |t, i, tm| {
                        Arc::new(
                            f.to_texture(i, Scene::get_texture(tm, &t.texture, i))
                                .unwrap(),
                        )
                    })
                }
                _ => (),
            };
        }
        let mut material_map = HashMap::<usize, Arc<dyn material::Material>>::default();
        let mut environment_map = HashMap::<usize, Arc<dyn environment::Environment>>::default();
        for obj in self.objects.iter() {
            match obj {
                RayTracingObject::NoMaterial(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(m.to_material(i).unwrap())
                    })
                }
                RayTracingObject::Lambertian(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(
                            m.to_material(i, Scene::get_texture(&texture_map, &m.albedo, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Metal(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(
                            m.to_material(i, Scene::get_texture(&texture_map, &m.albedo, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Dielectric(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(
                            m.to_material(i, Scene::get_texture(&texture_map, &m.albedo, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Isotropic(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(
                            m.to_material(i, Scene::get_texture(&texture_map, &m.albedo, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::DiffuseLight(m) => {
                    Scene::insert_material(&mut material_map, &m.id, m, |m, i, _| {
                        Arc::new(
                            m.to_material(i, Scene::get_texture(&texture_map, &m.emit, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Sky(s) => {
                    for index in 0..s.id.len() {
                        environment_map
                            .insert(s.id.get_id(index), Arc::new(s.to_environment(index)?));
                    }
                }
                _ => (),
            };
        }
        for obj in self.objects.iter() {
            match obj {
                RayTracingObject::MaterialBlend(m) => {
                    for index in 0..m.id.len() {
                        let v = m
                            .materials
                            .iter()
                            .map(|id| Scene::get_material(&material_map, id, index))
                            .collect();
                        material_map
                            .insert(m.id.get_id(index), Arc::new(m.to_material(index, &v)?));
                    }
                }
                _ => (),
            }
        }
        let mut object_map = HashMap::<usize, Arc<dyn geometry::Geometry>>::default();
        for obj in self.objects.iter() {
            match obj {
                RayTracingObject::Sphere(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::MovableSphere(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Collection(c) => {
                    let mut obj_list = Vec::<Arc<dyn geometry::Geometry>>::default();
                    for id in IdReference::get_list(&c.object_id_list) {
                        obj_list.push(Scene::get_geometry(
                            &object_map,
                            &IdReference::Single(id),
                            0,
                        ));
                    }
                    Scene::insert_geometry(&mut object_map, &c.id, c, |c, _, _| {
                        c.to_collection(&obj_list, 0.0..0.0).unwrap()
                    })
                }
                RayTracingObject::XYRect(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::XZRect(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::YZRect(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Cuboid(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, _| {
                        Arc::new(
                            h.to_shape(i, Scene::get_material(&material_map, &h.material, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::ConstantMedium(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_volume(
                                i,
                                Scene::get_geometry(&om, &h.boundary, i),
                                Scene::get_material(&material_map, &h.phase_function, i),
                            )
                            .unwrap(),
                        )
                    })
                }
                RayTracingObject::FlipNormals(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_geometry(i, Scene::get_geometry(&om, &h.node, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::RotateX(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_geometry(i, Scene::get_geometry(&om, &h.node, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::RotateY(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_geometry(i, Scene::get_geometry(&om, &h.node, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::RotateZ(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_geometry(i, Scene::get_geometry(&om, &h.node, i))
                                .unwrap(),
                        )
                    })
                }
                RayTracingObject::Translate(h) => {
                    Scene::insert_geometry(&mut object_map, &h.id, h, |h, i, om| {
                        Arc::new(
                            h.to_geometry(i, Scene::get_geometry(&om, &h.node, i))
                                .unwrap(),
                        )
                    })
                }
                _ => (),
            };
        }
        let light = if self.light_node_id > 0 {
            Some(Scene::get_geometry(
                &object_map,
                &IdReference::Single(self.light_node_id),
                0,
            ))
        } else {
            None
        };
        Ok(core::Scene::new(
            Scene::get_configuration(&configuration_map, &self.configuration_id),
            Scene::get_camera(&camera_map, &self.camera_id),
            Scene::get_environment(&environment_map, &self.sky_id),
            Scene::get_geometry(&object_map, &IdReference::Single(self.root_node_id), 0),
            light,
        ))
    }

    fn get_configuration(
        configuration_map: &HashMap<usize, core::Configuration>,
        id: &usize,
    ) -> core::Configuration {
        if !configuration_map.contains_key(id) {
            panic!("No configuration with id {} found", id);
        }
        configuration_map[id].clone()
    }

    fn get_camera(camera_map: &HashMap<usize, Arc<core::Camera>>, id: &usize) -> Arc<core::Camera> {
        if !camera_map.contains_key(id) {
            panic!("No camera with id {} found", id);
        }
        camera_map[id].clone()
    }

    fn get_environment(
        environment_map: &HashMap<usize, Arc<dyn environment::Environment>>,
        id: &usize,
    ) -> Arc<dyn environment::Environment> {
        if !environment_map.contains_key(id) {
            panic!("No environment with id {} found", id);
        }
        environment_map[id].clone()
    }

    fn get_texture(
        texture_map: &HashMap<usize, Arc<dyn texture::Texture>>,
        id: &IdReference,
        index: usize,
    ) -> Arc<dyn texture::Texture> {
        let id = id.get_id(index);
        if !texture_map.contains_key(&id) {
            panic!("No texture with id {} found", id);
        }
        texture_map[&id].clone()
    }

    fn get_material(
        material_map: &HashMap<usize, Arc<dyn material::Material>>,
        id: &IdReference,
        index: usize,
    ) -> Arc<dyn material::Material> {
        let id = id.get_id(index);
        if !material_map.contains_key(&id) {
            panic!("No material with id {} found", id);
        }
        material_map[&id].clone()
    }

    fn get_geometry(
        object_map: &HashMap<usize, Arc<dyn geometry::Geometry>>,
        id: &IdReference,
        index: usize,
    ) -> Arc<dyn geometry::Geometry> {
        let id = id.get_id(index);
        if !object_map.contains_key(&id) {
            panic!("No hit able object with id {} found", id);
        }
        object_map[&id].clone()
    }
}

#[cfg(test)]
mod serialize_scene_test {
    use super::*;
    use ray_tracing_core::test;

    #[test]
    fn serialize_scene_from_scene() {
        let ts = test::TestSceneSimple::new();
        let s = Scene::from_scene(&ts.scene).unwrap();
        assert_eq!(s.objects.len(), 10);
    }

    #[test]
    fn serialize_scene_to_scene() {
        let ts = test::TestSceneSimple::new();
        let s = Scene::from_scene(&ts.scene).unwrap();
        let ds = s.to_scene().unwrap();
        assert_eq!(ds.configuration.maximum_depth, 50);
    }
}
