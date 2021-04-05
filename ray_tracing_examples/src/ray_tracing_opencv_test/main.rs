use opencv::{core, highgui, imgproc};
use ray_tracing_core::test::TestSceneSimple;
use ray_tracing_core::types::FSize;
use ray_tracing_utility::iterator::IteratorExp2;

fn main() {
    let cx = 200;
    let cy = 100;
    let ray_tracer = TestSceneSimple::new().scene;
    let ray_trace_iter = IteratorExp2::new(cx, cy);

    let mut image: core::Mat = core::Mat::new_rows_cols_with_default(
        cy as i32,
        cx as i32,
        core::CV_8UC4,
        core::Value::from([0.0, 0.0, 0.0, 0.0]),
    )
    .unwrap();

    highgui::named_window("test", 0).unwrap();

    for (x, y, size) in ray_trace_iter {
        let u = x as FSize / cx as FSize;
        let v = 1.0 - y as FSize / cy as FSize;
        let c = ray_tracer.ray_trace_color(u, v);

        let color = core::Value::from([c[2] * 255.0, c[1] * 255.0, c[0] * 255.0, 255.0]);
        let rect = core::Rect {
            x: x as i32,
            y: y as i32,
            width: usize::min(size, cx - x) as i32,
            height: usize::min(size, cy - y) as i32,
        };
        imgproc::rectangle(&mut image, rect, color, -1, 8, 0).unwrap();

        highgui::imshow("test", &image).unwrap();
        //highgui::update_window("test").unwrap();
    }

    highgui::wait_key(0).unwrap();
}
