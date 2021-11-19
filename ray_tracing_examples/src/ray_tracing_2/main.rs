use ray_tracing_core::core::{Camera, Configuration, Scene};
use ray_tracing_core::environment::Sky;
use ray_tracing_core::geometry::collection::{BVHNode, GeometryList};
use ray_tracing_core::geometry::instancing::{FlipNormals, RotateY, Translate};
use ray_tracing_core::geometry::shape::{Cuboid, MovableSphere, Sphere, XZRect};
use ray_tracing_core::geometry::volume::ConstantMedium;
use ray_tracing_core::geometry::Geometry;
use ray_tracing_core::material::{Dielectric, DiffuseLight, Isotropic, Lambertian, Metal};
use ray_tracing_core::random;
use ray_tracing_core::texture::{ConstantTexture, NoiseTexture, NoiseType, Texture};
use ray_tracing_core::types::{ColorRGB, ColorRGBA, FSize, Point3, Vector3};
use ray_tracing_show_image;
use ray_tracing_utility::image;
use ray_tracing_utility::serialization::json;
use ray_tracing_utility::serialization::texture::BitmapFile;
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

    let default_view_model = ViewModel {
        cx: 800,
        cy: 800,
        repetitions_threads: 7,
        repetitions: 10000,
        samples: 10,
    };

    let view_model = match args.next() {
        Some(arg) => json::deserialize_view_model(&fs::read_to_string(arg)?)?,
        None => default_view_model,
    };
    let scene_name = "CoverSceneRT2";
    let target_root = "./temp";
    let target_file_name = format!(
        "{}_{}x{}_{}_samples",
        scene_name,
        view_model.cx,
        view_model.cy,
        view_model.repetitions * view_model.samples
    );

    let mut object_vec = Vec::<Arc<dyn Geometry>>::default();

    //auto ground = make_shared<lambertian>(color(0.48, 0.83, 0.53));
    //const int boxes_per_side = 20;
    //for (int i = 0; i < boxes_per_side; i++) {
    //    for (int j = 0; j < boxes_per_side; j++) {
    //        auto w = 100.0;
    //        auto x0 = -1000.0 + i*w;
    //        auto z0 = -1000.0 + j*w;
    //        auto y0 = 0.0;
    //        auto x1 = x0 + w;
    //        auto y1 = random_double(1,101);
    //        auto z1 = z0 + w;
    //
    //        boxes1.add(make_shared<box>(point3(x0,y0,z0), point3(x1,y1,z1), ground));
    //    }
    //}
    let mut list_1 = Vec::<Arc<dyn Geometry>>::default();
    for i in 0..20 {
        for j in 0..20 {
            let v_min = Vector3::new(
                -1000.0 + (i as FSize * 100.0),
                -0.0,
                -1000.0 + (j as FSize * 100.0),
            );
            let v_max = Vector3::new(
                v_min.x + 100.0,
                100.0 * random::generate_size() + 0.01,
                v_min.z + 100.0,
            );
            list_1.push(Arc::new(Cuboid::new(
                v_min..v_max,
                Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                    ColorRGBA::new(0.48, 0.83, 0.53, 1.0),
                )))),
            )));
        }
    }
    object_vec.push(BVHNode::new(&list_1, 0.0..1.0));

    //auto light = make_shared<diffuse_light>(color(7, 7, 7));
    //objects.add(make_shared<xz_rect>(123, 423, 147, 412, 554, light));
    let light_node = Arc::new(FlipNormals::new(Arc::new(XZRect::new(
        (123.0, 147.0)..(423.0, 412.0),
        554.0,
        Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(7.0, 7.0, 7.0, 1.0),
        )))),
    ))));
    object_vec.push(light_node.clone());

    //auto center1 = point3(400, 400, 200);
    //auto center2 = center1 + vec3(30,0,0);
    //auto moving_sphere_material = make_shared<lambertian>(color(0.7, 0.3, 0.1));
    //objects.add(make_shared<moving_sphere>(center1, center2, 0, 1, 50, moving_sphere_material));
    object_vec.push(Arc::new(MovableSphere::new(
        Point3::new(400.0, 400.0, 200.0)..Point3::new(430.0, 400.0, 200.0),
        0.0..1.0,
        50.0,
        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(0.7, 0.3, 0.1, 1.0),
        )))),
    )));

    // objects.add(make_shared<sphere>(point3(260, 150, 45), 50, make_shared<dielectric>(1.5)));
    let glass_sphere_node = Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(
            1.5..1.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        )),
    ));
    object_vec.push(glass_sphere_node.clone());

    //objects.add(make_shared<sphere>(point3(0, 150, 145), 50, make_shared<metal>(color(0.8, 0.8, 0.9), 1.0)));
    object_vec.push(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(
            1.0,
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.8, 0.8, 0.9, 1.0))),
        )),
    )));

    //auto boundary = make_shared<sphere>(point3(360,150,145), 70, make_shared<dielectric>(1.5));
    //objects.add(boundary);
    //objects.add(make_shared<constant_medium>(boundary, 0.2, color(0.2, 0.4, 0.9)));
    let boundary1 = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(
            1.5..1.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        )),
    ));
    object_vec.push(boundary1.clone());
    object_vec.push(Arc::new(ConstantMedium::new(
        0.2,
        boundary1,
        Arc::new(Isotropic::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(0.2, 0.4, 0.9, 1.0),
        )))),
    )));

    //boundary = make_shared<sphere>(point3(0, 0, 0), 5000, make_shared<dielectric>(1.5));
    //objects.add(make_shared<constant_medium>(boundary, .0001, color(1,1,1)));
    let boundary2 = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(
            1.5..1.5,
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        )),
    );
    object_vec.push(Arc::new(ConstantMedium::new(
        0.0001,
        Arc::new(boundary2),
        Arc::new(Isotropic::new(Arc::new(ConstantTexture::new(
            ColorRGBA::new(1.0, 1.0, 1.0, 1.0),
        )))),
    )));

    //auto emat = make_shared<lambertian>(make_shared<image_texture>("earthmap.jpg"));
    //objects.add(make_shared<sphere>(point3(400,200,400), 100, emat));
    let texture_object: Arc<dyn Texture> = match BitmapFile::file_to_texture(&String::from(
        "./resource/texture/physical-free-world-map-b1.jpg",
    )) {
        Ok(texture) => Arc::new(texture),
        Err(_) => Arc::new(NoiseTexture::new(
            0.05,
            NoiseType::Turb,
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.7, 0.3, 0.1, 1.0))),
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.1, 0.3, 0.7, 1.0))),
        )),
    };
    object_vec.push(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Lambertian::new(texture_object)),
    )));

    //auto pertext = make_shared<noise_texture>(0.1);
    //objects.add(make_shared<sphere>(point3(220,280,300), 80, make_shared<lambertian>(pertext)));
    object_vec.push(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(
            0.1,
            NoiseType::SinZ,
            Arc::new(ConstantTexture::new(ColorRGBA::new(0.0, 0.0, 0.0, 1.0))),
            Arc::new(ConstantTexture::new(ColorRGBA::new(1.0, 1.0, 1.0, 1.0))),
        )))),
    )));

    //hittable_list boxes2;
    //auto white = make_shared<lambertian>(color(.73, .73, .73));
    //int ns = 1000;
    //for (int j = 0; j < ns; j++) {
    //    boxes2.add(make_shared<sphere>(point3::random(0,165), 10, white));
    //}
    //
    //objects.add(make_shared<translate>(
    //    make_shared<rotate_y>(
    //        make_shared<bvh_node>(boxes2, 0.0, 1.0), 15),
    //        vec3(-100,270,395)
    //    )
    //);
    let mut list_2 = Vec::<Arc<dyn Geometry>>::default();
    for _ in 0..1000 {
        list_2.push(Arc::new(Sphere::new(
            Vector3::new(
                165.0 * random::generate_size(),
                165.0 * random::generate_size(),
                165.0 * random::generate_size(),
            ),
            10.0,
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
                ColorRGBA::new(0.73, 0.73, 0.73, 1.0),
            )))),
        )));
    }
    object_vec.push(Arc::new(Translate::new(
        Vector3::new(-100.0, 270.0, 395.0),
        Arc::new(RotateY::new(
            FSize::to_radians(15.0),
            BVHNode::new(&list_2, 0.0..1.0),
        )),
    )));

    //lookfrom = point3(478, 278, -600);
    //lookat = point3(278, 278, 0);
    //vfov = 40.0;
    let lights: Vec<Arc<dyn Geometry>> = vec![light_node, glass_sphere_node];
    let scene = Scene::new(
        Configuration::default(),
        Arc::new(Camera::from_look_at(
            Vector3::new(478.0, 278.0, -600.0),
            Vector3::new(278.0, 278.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            40.0,
            2.0,
            0.0,
            10.0,
            0.0..1.0,
        )),
        Arc::new(Sky::new(
            ColorRGB::new(0.0, 0.0, 0.0),
            ColorRGB::new(0.0, 0.0, 0.0),
        )),
        BVHNode::new(&object_vec, 0.0..1.0),
        Some(Arc::new(GeometryList::new(&lights))),
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
