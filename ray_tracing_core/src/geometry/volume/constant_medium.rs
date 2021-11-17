use crate::core::object::Object;
use crate::core::HitRecord;
use crate::geometry::{Geometry, Visitor};
use crate::material::Material;
use crate::math::{Ray, AABB};
use crate::random;
use crate::types::{FSize, TextureCoordinate, Vector3};
use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

pub struct ConstantMedium {
    pub id: usize,
    pub density: FSize,
    pub boundary: Arc<dyn Geometry>,
    pub phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(
        density: FSize,
        boundary: Arc<dyn Geometry>,
        phase_function: Arc<dyn Material>,
    ) -> ConstantMedium {
        ConstantMedium {
            id: Object::new_id(),
            density,
            boundary,
            phase_function,
        }
    }
}

impl Geometry for ConstantMedium {
    fn get_id(&self) -> usize {
        self.id
    }

    fn bounding_box(&self, time: Range<FSize>) -> Option<AABB> {
        self.boundary.bounding_box(time)
    }

    fn hit(&self, ray: &Ray, t_range: Range<FSize>) -> Option<HitRecord> {
        //bool enableDebug = false;
        //bool db = enableDebug && _sampler.Next() < 0.00001;

        match self.boundary.hit(ray, FSize::MIN..FSize::MAX) {
            Some(hit_record) => {
                let mut record_1 = HitRecord::from_hit_record(&hit_record);
                match self.boundary.hit(ray, record_1.t + 0.0001..FSize::MAX) {
                    Some(mut record_2) => {
                        //if (enableDebug)
                        //    Console.WriteLine($"t0 t1 {rec1.T} {rec2.T}");

                        if record_1.t < t_range.start {
                            record_1.t = t_range.start;
                        }
                        if record_2.t > t_range.end {
                            record_2.t = t_range.end;
                        }

                        if record_1.t >= record_2.t {
                            return None;
                        }
                        if record_1.t < 0.0 {
                            record_1.t = 0.0;
                        }

                        let distance_inside_boundary =
                            (record_2.t - record_1.t) * glm::length(ray.direction);
                        let hit_distance = -1.0 / self.density * FSize::ln(random::generate_size());
                        if hit_distance < distance_inside_boundary {
                            let t = record_1.t + hit_distance / glm::length(ray.direction);
                            let p = ray.point_at(hit_record.t);
                            //if (enableDebug)
                            //    Console.WriteLine($"hit_distance {hit_distance}; rec.T {rec.T}; rectP {rec.P}");
                            return Some(HitRecord::new(
                                t,
                                TextureCoordinate::from_uv(0.0, 0.0),
                                p,
                                Vector3::new(1.0, 0.0, 0.0), // arbitrary
                                self.phase_function.clone(),
                                self.phase_function
                                    .color_channels(&TextureCoordinate::from_uv(0.0, 0.0), &p),
                            ));
                        }
                        None
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>> {
        visitor.visit_volume_constant_medium(&self)
    }
}

#[cfg(test)]
mod constant_medium_test {
    use super::*;
    use crate::geometry::shape::Sphere;
    use crate::material::{Lambertian, Metal, NoMaterial};
    use crate::test;
    use crate::texture::ConstantTexture;
    use crate::types::ColorRGBA;
    use crate::types::{Point3, Vector3};

    #[test]
    fn bounding_box_test() {
        let s = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, Arc::new(NoMaterial::new()));
        let i = ConstantMedium::new(0.0, Arc::new(s), Arc::new(NoMaterial::new()));
        let b = i.bounding_box(0.0..0.0);
        match b {
            Some(b) => {
                test::assert_eq_vector3(&b.min, &Vector3::new(-1.0, -1.0, -1.0), 0.01);
                test::assert_eq_vector3(&b.max, &Vector3::new(1.0, 1.0, 1.0), 0.01);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn hit_test() {
        let s = Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(Metal::new(
                0.0,
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            )),
        );
        let i = ConstantMedium::new(
            0.5,
            Arc::new(s),
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
            )))),
        );
        let ray1 = Ray::new_ray(Vector3::new(0.0, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new_ray(Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        match i.hit(&ray1, 0.0..10.0) {
            Some(_) => {}
            None => {}
        }
        match i.hit(&ray1, 10.0..20.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
        match i.hit(&ray2, 0.0..10.0) {
            Some(_) => panic!("unexpected hit"),
            None => (),
        }
    }
}
