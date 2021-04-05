use crate::iterator::IteratorExp2;
use crate::thread::RayTraceProcess;
use crate::view;
use crate::view::View;
use ray_tracing_core::core::Scene;
use ray_tracing_core::types::{ColorRGB, FSize};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ops::Fn;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ViewModel {
    pub cx: usize,
    pub cy: usize,
    pub repetitions_threads: usize,
    pub repetitions: usize,
    pub samples: usize,
}

pub struct Viewer {
    view_model: ViewModel,
    ray_tracer: RayTraceProcess,
    view: Arc<dyn View>,
    fn_save: Box<dyn Fn(usize, usize, usize, &Vec<u8>) -> ()>,
}

impl Viewer {
    pub fn new(
        view_model: ViewModel,
        scene: Arc<Scene>,
        view: Arc<dyn View>,
        fn_save: Box<dyn Fn(usize, usize, usize, &Vec<u8>) -> ()>,
    ) -> Result<Viewer, Box<dyn Error>> {
        let ray_tracer = RayTraceProcess::new(
            view_model.cx,
            view_model.cy,
            view_model.repetitions_threads,
            view_model.repetitions,
            view_model.samples,
            Arc::new(scene.from_scene_and_aspect(view_model.cx as FSize / view_model.cy as FSize)?),
            Arc::new(Mutex::new(IteratorExp2::new(view_model.cx, view_model.cy))),
        );
        Ok(Viewer {
            view_model,
            ray_tracer,
            view: view.clone(),
            fn_save: Box::new(fn_save),
        })
    }

    pub fn run(&mut self) -> Result<(usize, usize, Vec<u8>), Box<dyn Error>> {
        let mut pixel_data: Vec<u8> =
            Vec::with_capacity(self.view_model.cx * self.view_model.cy * 4);
        pixel_data.resize(self.view_model.cx * self.view_model.cy * 4, 0);
        let mut sample_count: Vec<i32> =
            Vec::with_capacity(self.view_model.cx * self.view_model.cy);
        sample_count.resize(self.view_model.cx * self.view_model.cy, -1);
        let mut pixel_color: Vec<ColorRGB> =
            Vec::with_capacity(self.view_model.cx * self.view_model.cy);
        pixel_color.resize(
            self.view_model.cx * self.view_model.cy,
            ColorRGB::new(0.0, 0.0, 0.0),
        );

        let start_time = SystemTime::now();
        let mut update_duration = Duration::from_secs(1);

        self.ray_tracer.start();

        let expected_results =
            self.view_model.cx * self.view_model.cy * self.view_model.repetitions;
        let mut received_results = 0;
        let mut image_number = 0;
        let mut exit = false;
        loop {
            match self.ray_tracer.next() {
                Some(result) => {
                    let i = (result.y * self.view_model.cx) + result.x;
                    if sample_count[i] < 0 {
                        for ix in result.x..usize::min(result.x + result.size, self.view_model.cx) {
                            for iy in
                                result.y..usize::min(result.y + result.size, self.view_model.cy)
                            {
                                let inner_i = (iy * self.view_model.cx) + ix;
                                if sample_count[inner_i] < 0 {
                                    pixel_color[inner_i] = result.color;
                                    sample_count[i] = if i == inner_i { 1 } else { 0 };
                                    pixel_data[inner_i * 4] =
                                        (result.color[0].sqrt() * 255.0).round() as u8;
                                    pixel_data[inner_i * 4 + 1] =
                                        (result.color[1].sqrt() * 255.0).round() as u8;
                                    pixel_data[inner_i * 4 + 2] =
                                        (result.color[2].sqrt() * 255.0).round() as u8;
                                    pixel_data[inner_i * 4 + 3] = 255;
                                }
                            }
                        }
                    } else {
                        if sample_count[i] < 0 {
                            sample_count[i] = 0;
                        }
                        let w = result.samples as FSize
                            / (sample_count[i] + result.samples as i32) as FSize;
                        let c = pixel_color[i] * (1.0 - w) + result.color * w;
                        pixel_color[i] = c;
                        sample_count[i] += result.samples as i32;
                        pixel_data[i * 4] = (c[0].sqrt() * 255.0).round() as u8;
                        pixel_data[i * 4 + 1] = (c[1].sqrt() * 255.0).round() as u8;
                        pixel_data[i * 4 + 2] = (c[2].sqrt() * 255.0).round() as u8;
                        pixel_data[i * 4 + 3] = 255;
                        received_results += 1;
                    }
                }
                None => (),
            }

            match self.view.handle_events() {
                Ok(view::Event::Close) => exit = true,
                Ok(view::Event::Save) => {
                    (*self.fn_save)(
                        image_number,
                        self.view_model.cx,
                        self.view_model.cy,
                        &pixel_data,
                    );
                    image_number += 1;
                }
                Ok(_) => (),
                Err(_) => exit = true,
            }
            if exit {
                break;
            }
            if start_time.elapsed().unwrap() >= update_duration {
                let finished = self.ray_tracer.finished();
                update_duration += Duration::from_secs(1);
                self.view.update(&pixel_data)?;
                if finished {
                    break;
                }
                println!("{}", received_results as f32 / expected_results as f32);
            }
        }

        if exit {
            Err("abort".into())
        } else {
            Ok((self.view_model.cx, self.view_model.cy, pixel_data))
        }
    }
}
