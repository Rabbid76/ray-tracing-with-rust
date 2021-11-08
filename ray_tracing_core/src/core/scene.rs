use crate::core::{Camera, Configuration};
use crate::environment::Environment;
use crate::geometry::Geometry;
use crate::math::Ray;
use crate::probability_density_function::{GeometryPdf, MixturePdf, ProbabilityDensityFunction};
use crate::types::{ColorRGB, FSize};
use std::error::Error;
use std::sync::Arc;

pub struct Scene {
    pub configuration: Configuration,
    pub camera: Arc<Camera>,
    pub sky: Arc<dyn Environment>,
    pub world: Arc<dyn Geometry>,
    pub light: Option<Arc<dyn Geometry>>,
}

impl Scene {
    pub fn new(
        configuration: Configuration,
        camera: Arc<Camera>,
        sky: Arc<dyn Environment>,
        world: Arc<dyn Geometry>,
        light: Option<Arc<dyn Geometry>>,
    ) -> Scene {
        Scene {
            configuration,
            camera,
            sky,
            world,
            light,
        }
    }

    pub fn ray_trace_color(&self, u: FSize, v: FSize) -> ColorRGB {
        let color = self.ray_trace_color_loop(u, v, self.light.clone());
        if color.x.is_nan() || color.y.is_nan() || color.z.is_nan() {
            ColorRGB::new(0.0, 0.0, 0.0)
        } else {
            color
        }
    }

    pub fn ray_trace_color_loop(
        &self,
        u: FSize,
        v: FSize,
        light_shape: Option<Arc<dyn Geometry>>,
    ) -> ColorRGB {
        let mut ray = self.camera.get(u, v);
        let mut color = ColorRGB::new(0.0, 0.0, 0.0);
        let mut attenuation = ColorRGB::new(1.0, 1.0, 1.0);
        for _ in 0..self.configuration.maximum_depth {
            if let Some(hit_record) = self.world.hit(&ray, 0.001..FSize::MAX) {
                let material = hit_record.material.clone();
                let emitted = material.emitted(&ray, &hit_record);
                color = color + attenuation * emitted;
                if let Some(scatter_record) = hit_record.scatter(&ray) {
                    if scatter_record.is_specular {
                        attenuation = attenuation * scatter_record.attenuation;
                        ray = scatter_record.ray;
                    } else {
                        let pdf: Option<Arc<dyn ProbabilityDensityFunction>> =
                            match scatter_record.pdf {
                                Some(pdf) => match light_shape {
                                    Some(ref light_shape) => Some(Arc::new(MixturePdf::new(
                                        pdf.clone(),
                                        Arc::new(GeometryPdf::new(
                                            &hit_record.position,
                                            light_shape.clone(),
                                        )),
                                    ))),
                                    None => Some(pdf.clone()),
                                },
                                None => None,
                            };
                        match pdf {
                            Some(pdf) => {
                                let scattered = Ray::new_ray_with_attributes(
                                    hit_record.position,
                                    pdf.generate(),
                                    &ray,
                                );
                                let s_pdf = scatter_record.material.scattering_pdf(
                                    &ray,
                                    &hit_record,
                                    &scattered,
                                );
                                let pdf_value = pdf.value(&scattered.direction);
                                attenuation =
                                    attenuation * scatter_record.attenuation * s_pdf / pdf_value;
                                ray = scatter_record.ray;
                            }
                            None => {
                                let s_pdf = scatter_record.material.scattering_pdf(
                                    &ray,
                                    &hit_record,
                                    &scatter_record.ray,
                                );
                                attenuation = attenuation * scatter_record.attenuation * s_pdf;
                                ray = scatter_record.ray;
                            }
                        }
                    }
                    //if attenuation.x + attenuation.y + attenuation.z < 0.00001 {
                    //    break;
                    //}
                } else {
                    break;
                }
            } else {
                color = color + attenuation * self.sky.get(&ray);
                break;
            }
        }
        color
    }

    pub fn ray_trace_color_recursive(&self, u: FSize, v: FSize) -> ColorRGB {
        let ray = self.camera.get(u, v);
        self.get_ray_trace_color(&ray, 0)
    }

    fn get_ray_trace_color(&self, ray: &Ray, depth: usize) -> ColorRGB {
        match self.world.hit(ray, 0.001..FSize::MAX) {
            Some(hit_record) => {
                let material = hit_record.material.clone();
                let mut emitted = material.emitted(&ray, &hit_record);
                if depth < self.configuration.maximum_depth {
                    match hit_record.scatter(ray) {
                        Some(scatter_record) => {
                            if scatter_record.is_specular {
                                emitted = emitted
                                    + scatter_record.attenuation
                                        * self.get_ray_trace_color(&scatter_record.ray, depth + 1);
                            } else {
                                let s_pdf = scatter_record.material.scattering_pdf(
                                    &ray,
                                    &hit_record,
                                    &scatter_record.ray,
                                );
                                emitted = emitted
                                    + scatter_record.attenuation
                                        * self.get_ray_trace_color(&scatter_record.ray, depth + 1)
                                        * s_pdf;
                            }
                        }
                        _ => (),
                    }
                }
                emitted
            }
            None => self.sky.get(ray),
        }
    }

    pub fn change_aspect(&mut self, aspect: FSize) {
        let mut c = (*self.camera).clone();
        c.change_aspect(aspect);
        self.camera = Arc::new(c);
    }

    pub fn from_scene_and_aspect(&self, aspect: FSize) -> Result<Scene, Box<dyn Error>> {
        let mut c = (*self.camera).clone();
        c.change_aspect(aspect);
        Ok(Scene::new(
            self.configuration.clone(),
            Arc::new(c),
            self.sky.clone(),
            self.world.clone(),
            self.light.clone(),
        ))
    }
}

#[cfg(test)]
mod scene_test {
    use super::*;
    use crate::random;
    use crate::test::TestSceneSimple;
    use std::ops::Range;

    fn assert_in_range(
        pixel_data: &Vec<u8>,
        cx: usize,
        _cy: usize,
        x: usize,
        y: usize,
        c: usize,
        range: Range<i32>,
    ) {
        println!(
            "{}..{}, {}",
            range.start,
            range.end,
            pixel_data[(y * cx + x) * 4 + c] as i32
        );
        assert!(range.contains(&(pixel_data[(y * cx + x) * 4 + c] as i32)));
    }

    #[test]
    fn render_simple_scene_test() {
        let cx = 20;
        let cy = 10;
        let samples = 1000;
        let scene = TestSceneSimple::new().scene;

        let mut pixel_data: Vec<u8> = Vec::with_capacity(cx * cy * 4);
        pixel_data.resize(cx * cy * 4, 0);

        for x in 0..cx {
            for y in 0..cy {
                let mut c = ColorRGB::new(0.0, 0.0, 0.0);
                for _ in 0..samples {
                    let u = (x as FSize + random::generate_size()) / cx as FSize;
                    let v = 1.0 - (y as FSize + random::generate_size()) / cy as FSize;
                    c = c + scene.ray_trace_color(u, v);
                }
                c = c / samples as FSize;

                let i = (y * cx) + x;
                pixel_data[i * 4] = (c[0].sqrt() * 255.0).round() as u8;
                pixel_data[i * 4 + 1] = (c[1].sqrt() * 255.0).round() as u8;
                pixel_data[i * 4 + 2] = (c[2].sqrt() * 255.0).round() as u8;
                pixel_data[i * 4 + 3] = 255;
            }
        }

        assert_in_range(&pixel_data, cx, cy, 3, 1, 2, 245..256);
        assert_in_range(&pixel_data, cx, cy, 10, 3, 0, 110..150);
        assert_in_range(&pixel_data, cx, cy, 17, 8, 1, 60..75);
    }
}
