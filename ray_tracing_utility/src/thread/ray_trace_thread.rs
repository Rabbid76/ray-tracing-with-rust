use crate::iterator::ViewportIterator;
use ray_tracing_core::core::Scene;
use ray_tracing_core::random;
use ray_tracing_core::types::{ColorRGB, FSize};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct RayTraceResult {
    pub x: usize,
    pub y: usize,
    pub samples: usize,
    pub size: usize,
    pub color: ColorRGB,
}

impl RayTraceResult {
    pub fn new(x: usize, y: usize, samples: usize, size: usize, color: ColorRGB) -> RayTraceResult {
        RayTraceResult {
            x,
            y,
            samples,
            size,
            color,
        }
    }
}

#[derive(Clone)]
pub struct Viewport {
    pub cx: usize,
    pub cy: usize,
}

#[derive(Clone)]
pub struct RayTraceThreadData {
    pub viewport: Viewport,
    pub scene: Arc<Scene>,
    pub iterator: Arc<Mutex<dyn ViewportIterator>>,
    pub tx: Sender<RayTraceResult>,
}

pub struct RayTraceThread {
    pub handle: JoinHandle<()>,
    pub finished: Arc<Mutex<bool>>,
}

impl RayTraceThread {
    pub fn new(thread_data: &RayTraceThreadData, samples: usize) -> RayTraceThread {
        let iterator = thread_data.iterator.clone();
        let scene = thread_data.scene.clone();
        let viewport = thread_data.viewport.clone();
        let tx = thread_data.tx.clone();
        let finished = Arc::new(Mutex::new(false));
        let finished_val = finished.clone();
        let handle = thread::spawn(move || {
            let iterator = iterator.lock().unwrap().create_new();
            let mut iterator = iterator.lock().unwrap();
            loop {
                match iterator.next() {
                    Some((x, y, size)) => {
                        let color = RayTraceThread::render(scene.clone(), &viewport, samples, x, y);
                        let result = RayTraceResult::new(x, y, samples, size, color);
                        tx.send(result).unwrap();
                    }
                    None => {
                        break;
                    }
                };
            }
            *finished_val.lock().unwrap() = true;
        });
        RayTraceThread { handle, finished }
    }

    fn render(
        scene: Arc<Scene>,
        viewport: &Viewport,
        samples: usize,
        x: usize,
        y: usize,
    ) -> ColorRGB {
        let mut c = ColorRGB::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u = (x as FSize + random::generate_size()) / viewport.cx as FSize;
            let v = 1.0 - (y as FSize + random::generate_size()) / viewport.cy as FSize;
            c = c + scene.ray_trace_color(u, v);
        }
        c / samples as FSize
    }
}
