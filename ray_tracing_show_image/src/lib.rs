//! # Crate ray_tracing_show_image
//! 
//! GitHub page [rabbid76.github.io/ray-tracing-with-rust](https://rabbid76.github.io/ray-tracing-with-rust/)  
//! GitHub repository [Rabbid76/ray-tracing-with-rust](https://github.com/Rabbid76/ray-tracing-with-rust)
//! 
//! [![](https://stackexchange.com/users/flair/7322082.png)](https://stackoverflow.com/users/5577765/rabbid76?tab=profile)
//! 
//! A very simple viewer of ray tracing progress
//! 
//! Shows the ray tracing progress in a window. The window is updated every second.
//! When "F1" is pressed, a callback is invoked with the intermediate rendering result.
//! This can be used to save the rendering.
//! The returned image is in RGB format and is stored in a tightly packed `Vec<u8>` object.
//!
//! # Examples
//!
//! ```rust
//! use ray_tracing_core::test::TestSceneSimple;
//! use ray_tracing_utility::view;
//! use ray_tracing_utility::view::{Viewer, ViewModel};
//! use std::error::Error;
//! use std::sync::Arc;
//!
//! #[show_image::main]
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let view_model = ViewModel {
//!         cx: 400,
//!         cy: 200,
//!         repetitions_threads: 2,
//!         repetitions: 5,
//!         samples: 2,
//!     };
//!     let window = ray_tracing_show_image::ShowImageWindow::new(view_model.cx, view_model.cy);
//!     let mut viewer = Viewer::new(
//!         view_model,
//!         Arc::new(TestSceneSimple::new().scene),
//!         window.clone(),
//!         Box::new(|image_number, cx, cy, pixel_data| {
//!
//!             // Save the intermediate rendering here ...
//!         }),
//!     )?;
//!
//!     match viewer.run() {
//!         Ok((cx, cy, pixel_data)) => {
//!             
//!             // Save the final rendering here ...
//!         }
//!         _ => (),
//!     }
//!
//!     Ok(())
//! }
//! ```
//! 
//! ![TestSceneSimple_800x400_10000_samples](https://raw.githubusercontent.com/Rabbid76/ray-tracing-with-rust/main/rendering/TestSceneSimple_800x400_10000_samples.png)

use ray_tracing_utility::view;
use show_image::event;
use show_image::{ImageInfo, ImageView, WindowOptions, WindowProxy};
use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

pub struct ShowImageWindow {
    cx: usize,
    cy: usize,
    window: WindowProxy,
    event_channel: Rc<Receiver<event::WindowEvent>>,
}

impl ShowImageWindow {
    pub fn new(cx: usize, cy: usize) -> Arc<dyn view::View> {
        let window_options = WindowOptions {
            preserve_aspect_ratio: true,
            background_color: show_image::Color::rgb(0.0, 0.0, 0.0),
            start_hidden: false,
            size: Some([cx as u32, cy as u32]),
            resizable: false,
            borderless: false,
            show_overlays: false,
        };
        let window = show_image::create_window("ray trace", window_options).unwrap();
        let event_channel = Rc::new(window.event_channel().unwrap());
        Arc::new(ShowImageWindow {
            cx,
            cy,
            window,
            event_channel,
        })
    }

    fn handle_event(&self, event: event::WindowEvent) -> Result<view::Event, Box<dyn Error>> {
        if let event::WindowEvent::CloseRequested(_) = event {
            println!("window closed");
            return Ok(view::Event::Close);
        }
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.state.is_pressed() {
                if event.input.key_code == Some(event::VirtualKeyCode::Escape) {
                    println!("ESC");
                    return Ok(view::Event::Close);
                }
                if event.input.key_code == Some(event::VirtualKeyCode::F1) {
                    return Ok(view::Event::Save);
                }
            }
        }
        Ok(view::Event::None)
    }

    pub fn event_loop(&self) -> Result<view::Event, Box<dyn Error>> {
        // Print keyboard events until Escape is pressed, then exit.
        // If the user closes the window, the channel is closed and the loop also exits.
        for event in self.window.event_channel().unwrap() {
            match self.handle_event(event) {
                Ok(event) => {
                    if event != view::Event::None {
                        return Ok(event);
                    }
                }
                Err(_) => break,
            }
        }
        Err("error".into())
    }
}

impl view::View for ShowImageWindow {
    fn update(&self, pixel_data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let image = ImageView::new(ImageInfo::rgba8(self.cx as u32, self.cy as u32), pixel_data);
        self.window.set_image("image-001", image)?;
        Ok(())
    }

    fn handle_events(&self) -> Result<view::Event, Box<dyn Error>> {
        match self.event_channel.try_recv() {
            Ok(event) => self.handle_event(event),
            _ => Ok(view::Event::None),
        }
    }
}
