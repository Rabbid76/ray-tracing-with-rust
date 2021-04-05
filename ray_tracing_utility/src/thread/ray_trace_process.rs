use crate::iterator::ViewportIterator;
use crate::thread::{RayTraceResult, RayTraceThread, RayTraceThreadData, Viewport};
use ray_tracing_core::core::Scene;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct RayTraceProcess {
    thread_data: RayTraceThreadData,
    repetitions_threads: usize,
    repetitions: usize,
    samples: usize,
    rx: Receiver<RayTraceResult>,
    thread_handles: Vec<JoinHandle<()>>,
    finished: Arc<Mutex<bool>>,
}

impl RayTraceProcess {
    pub fn new(
        cx: usize,
        cy: usize,
        repetitions_threads: usize,
        repetitions: usize,
        samples: usize,
        scene: Arc<Scene>,
        iterator: Arc<Mutex<dyn ViewportIterator>>,
    ) -> RayTraceProcess {
        let (tx, rx) = mpsc::channel::<RayTraceResult>();
        RayTraceProcess {
            thread_data: RayTraceThreadData {
                viewport: Viewport { cx, cy },
                scene,
                iterator,
                tx,
            },
            repetitions_threads,
            repetitions,
            samples,
            rx,
            thread_handles: Vec::default(),
            finished: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self) {
        let repetitions_threads = self.repetitions_threads;
        let repetitions = self.repetitions;
        let samples = self.samples;
        let thread_data = self.thread_data.clone();
        let finished = self.finished.clone();
        self.thread_handles.push(thread::spawn(move || {
            let first_rough_thread = RayTraceThread::new(&thread_data, 1);
            first_rough_thread.handle.join().unwrap();
            for _ in 0..repetitions / repetitions_threads {
                let mut threads = Vec::default();
                for _ in 0..repetitions_threads {
                    threads.push(RayTraceThread::new(&thread_data, samples));
                }
                for thread in threads {
                    thread.handle.join().unwrap();
                }
            }
            *finished.lock().unwrap() = true;
        }));
    }

    pub fn finished(&self) -> bool {
        *self.finished.lock().unwrap()
    }
}

impl Iterator for RayTraceProcess {
    type Item = RayTraceResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rx.try_recv() {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}
