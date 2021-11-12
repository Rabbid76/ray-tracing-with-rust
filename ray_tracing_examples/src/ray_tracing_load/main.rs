use ray_tracing_show_image;
use ray_tracing_utility::image;
use ray_tracing_utility::serialization::core::DeserializeOptions;
use ray_tracing_utility::serialization::json;
use ray_tracing_utility::view;
use ray_tracing_utility::view::{ViewModel, Viewer};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;

/// ```lang-none
/// cargo run --bin rt_load scene/TestSceneSimple.json scene/TestConfiguration.json
/// ```
#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let default_file_path = "scene/TestSceneSimple.json";
    let default_view_model = ViewModel {
        cx: 400,
        cy: 200,
        repetitions_threads: 2,
        repetitions: 100,
        samples: 10,
    };
    let mut args = env::args();
    args.next();
    let file_path = match args.next() {
        Some(arg) => arg,
        None => String::from(default_file_path),
    };
    let view_model = match args.next() {
        Some(arg) => json::deserialize_view_model(&fs::read_to_string(arg)?)?,
        None => default_view_model,
    };
    let mut json_dir = env::current_dir()?;
    match Path::new(&file_path).parent() {
        Some(path) => {
            if path.is_absolute() {
                json_dir = path.to_path_buf();
            } else {
                json_dir = json_dir.join(path);
            }
        }
        None => (),
    }
    let file_name = Path::new(&file_path).file_stem().unwrap().to_str().unwrap();
    let target_root = "./temp";
    let target_file_name = format!(
        "{}_{}x{}_{}_samples",
        file_name,
        view_model.cx,
        view_model.cy,
        view_model.repetitions * view_model.samples
    );

    let json_scene = fs::read_to_string(file_path)?;
    let window = ray_tracing_show_image::ShowImageWindow::new(view_model.cx, view_model.cy);
    let test_file_name = format!("{}/{}_test_", target_root, target_file_name);
    let options = DeserializeOptions::form_path(json_dir.as_path());
    let mut viewer = Viewer::new(
        view_model,
        Arc::new(json::deserialize_scene_with_options(&json_scene, &options)?),
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
