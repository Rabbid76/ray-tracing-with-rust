use ray_tracing_core::core::{Camera, Configuration, Scene};
use ray_tracing_core::environment::Sky;
use ray_tracing_core::hit_able::collection::BVHNode;
use ray_tracing_core::hit_able::shape::{MovableSphere, Sphere};
use ray_tracing_core::hit_able::HitAble;
use ray_tracing_core::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing_core::random;
use ray_tracing_core::texture::{CheckerTexture, ConstantTexture};
use ray_tracing_core::types;
use ray_tracing_core::types::{ColorRGB, ColorRGBA, Point3, Vector3};
use ray_tracing_show_image;
use ray_tracing_utility::image;
use ray_tracing_utility::serialization::json;
use ray_tracing_utility::view;
use ray_tracing_utility::view::{ViewModel, Viewer};
use std::env;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let motion = match args.next() {
        Some(arg) => arg == "motion",
        None => false,
    };

    let default_view_model = ViewModel {
        cx: 800,
        cy: 400,
        repetitions_threads: 2,
        repetitions: 1000,
        samples: 10,
    };

    let view_model = match args.next() {
        Some(arg) => json::deserialize_view_model(&fs::read_to_string(arg)?)?,
        None => default_view_model,
    };
    let scene_name = "CoverSceneRT1";
    let target_root = "c:/temp";
    let target_file_name = format!(
        "{}_{}x{}_{}_samples",
        scene_name,
        view_model.cx,
        view_model.cy,
        view_model.repetitions * view_model.samples
    );

    let mut object_vec = Vec::<Arc<dyn HitAble>>::default();

    object_vec.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(if motion.clone() {
            Arc::new(CheckerTexture::new(
                Vector3::new(10.0, 10.0, 10.0),
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.2, 0.3, 0.1, 1.0))),
                Arc::new(ConstantTexture::new(ColorRGBA::new(0.9, 0.9, 0.9, 1.0))),
            ))
        } else {
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.5, 0.5, 0.5, 1.0)))
        })),
    )));
    let cpt_s1 = Point3::new(0.0, 1.0, 0.0);
    object_vec.push(Arc::new(Sphere::new(
        cpt_s1,
        1.0,
        Arc::new(Dielectric::new(
            1.5..1.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        )),
    )));
    let cpt_s2 = Point3::new(-4.0, 1.0, 0.0);
    object_vec.push(Arc::new(Sphere::new(
        cpt_s2,
        1.0,
        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(0.4, 0.2, 0.1, 1.0),
        )))),
    )));
    let cpt_s3 = Point3::new(4.0, 1.0, 0.0);
    object_vec.push(Arc::new(Sphere::new(
        cpt_s3,
        1.0,
        Arc::new(Metal::new(
            0.1,
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.7, 0.6, 0.5, 1.0))),
        )),
    )));

    let mut spheres = vec![(cpt_s1, 1.0), (cpt_s2, 1.0), (cpt_s3, 1.0)];
    let m = Vector3::new(0.0, if motion { 0.5 } else { 0.0 }, 0.0);
    for _ in 0..400 {
        let cpt = loop {
            let c = Point3::new(
                random::generate_range(-10.0..10.0),
                0.2,
                random::generate_range(-10.0..10.0),
            );
            match spheres
                .iter()
                .find(move |(c2, r2)| types::distance_square(c + m, *c2) < (r2 + 0.2) * (r2 + 0.2))
            {
                Some(_) => (),
                None => {
                    break c;
                }
            }
        };
        spheres.push((cpt + m, 0.2));
        let random_material_select = random::generate_size();
        if random_material_select < 0.7 {
            let material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(
                ConstantTexture::new(ColorRGBA::new(
                    random::generate_size(),
                    random::generate_size(),
                    random::generate_size(),
                    1.0,
                )),
            )));
            if motion {
                let cpt2 = cpt + Vector3::new(0.0, 0.5 * random::generate_size(), 0.0);
                object_vec.push(Arc::new(MovableSphere::new(
                    cpt..cpt2,
                    0.0..1.0,
                    0.2,
                    material,
                )));
            } else {
                object_vec.push(Arc::new(Sphere::new(cpt, 0.2, material)));
            }
        } else if random_material_select < 0.9 {
            let material: Arc<dyn Material> = Arc::new(Metal::new(
                random::generate_range(0.5..1.0),
                Arc::new(ConstantTexture::new(ColorRGBA::new(
                    random::generate_range(0.5..1.0),
                    random::generate_range(0.5..1.0),
                    random::generate_range(0.5..1.0),
                    1.0,
                ))),
            ));
            object_vec.push(Arc::new(Sphere::new(cpt, 0.2, material)));
        } else {
            let ref_idx = random::generate_range(1.45..1.55);
            let material: Arc<dyn Material> = Arc::new(Dielectric::new(
                ref_idx..ref_idx,
                Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
            ));
            object_vec.push(Arc::new(Sphere::new(cpt, 0.2, material)));
        };
    }

    let scene = Scene::new(
        Configuration::default(),
        Arc::new(Camera::from_look_at(
            Vector3::new(12.0, 2.0, 3.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            20.0,
            2.0,
            0.1,
            10.0,
            0.0..1.0,
        )),
        Arc::new(Sky::new(
            ColorRGB::new(1.0, 1.0, 1.0),
            ColorRGB::new(0.5, 0.7, 1.0),
        )),
        BVHNode::new(&object_vec, 0.0..1.0),
        None,
    );

    let window = ray_tracing_show_image::ShowImageWindow::new(view_model.cx, view_model.cy);
    let test_file_name = format!("{}/{}_test_", target_root, target_file_name);
    let mut viewer = Viewer::new(
        view_model,
        Arc::new(scene),
        window.clone(),
        Box::new(move |image_number, cx, cy, data| {
            let file_name = format!("{}{}.png", test_file_name, image_number);
            image::save_image(&file_name, cx, cy, data);
            println!("saved {}", file_name);
        }),
    )?;

    println!("start");
    let start_time = SystemTime::now();

    match viewer.run() {
        Ok((cx, cy, pixel_data)) => {
            let elapsed_time = start_time.elapsed();
            println!("end");
            match elapsed_time {
                Ok(elapsed) => {
                    println!(
                        "rendered in {} seconds",
                        elapsed.as_millis() as f64 / 1000.0
                    );
                }
                Err(_) => (),
            }

            let file_name = format!("{}/{}.png", target_root, target_file_name);
            image::save_image(&file_name, cx, cy, &pixel_data);
            println!("saved {}", file_name);

            loop {
                match window.handle_events() {
                    Ok(view::Event::Close) => break,
                    Ok(_) => (),
                    Err(_) => break,
                }
            }
        }
        _ => (),
    }

    Ok(())
}
