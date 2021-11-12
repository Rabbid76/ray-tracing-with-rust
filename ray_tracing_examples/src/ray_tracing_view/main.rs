use ray_tracing_core::test::TestSceneSimple;
use ray_tracing_show_image;
use ray_tracing_utility::image;
use ray_tracing_utility::view;
use ray_tracing_utility::view::{ViewModel, Viewer};
use std::error::Error;
use std::sync::Arc;
use std::time::SystemTime;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let view_model = ViewModel {
        cx: 400,
        cy: 200,
        repetitions_threads: 2,
        repetitions: 100,
        samples: 10,
    };
    let window = ray_tracing_show_image::ShowImageWindow::new(view_model.cx, view_model.cy);
    let mut viewer = Viewer::new(
        view_model,
        Arc::new(TestSceneSimple::new().scene),
        window.clone(),
        Box::new(|image_number, cx, cy, data| {
            let mut file_name = "./temp/test_".to_owned();
            file_name.push_str(&image_number.to_string());
            file_name.push_str(".png");
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

            let file_name = "c:/temp/test_final.png";
            image::save_image(file_name, cx, cy, &pixel_data);
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
