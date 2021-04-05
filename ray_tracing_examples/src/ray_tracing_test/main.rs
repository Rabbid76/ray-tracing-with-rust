use ray_tracing_core::random;
use ray_tracing_core::test::TestSceneSimple;
use ray_tracing_core::types::ColorRGB;
use ray_tracing_core::types::FSize;
use ray_tracing_utility::image;
use ray_tracing_utility::iterator::IteratorExp2;

fn main() {
    let cx = 200;
    let cy = 100;
    let samples = 100;
    let scene = TestSceneSimple::new().scene;
    let ray_trace_iter = IteratorExp2::new(cx, cy);

    let mut pixel_data: Vec<u8> = Vec::with_capacity(cx * cy * 4);
    pixel_data.resize(cx * cy * 4, 0);

    for (x, y, _size) in ray_trace_iter {
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

    let file_name = "c:/temp/rt_test.png";
    image::save_image(file_name, cx, cy, &pixel_data);
}
