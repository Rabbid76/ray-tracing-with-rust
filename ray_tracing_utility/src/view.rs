use std::error::Error;

mod viewer;
pub use self::viewer::{ViewModel, Viewer};

#[derive(PartialEq)]
pub enum Event {
    None,
    Close,
    Save,
}

pub trait View {
    fn update(&self, pixel_data: &Vec<u8>) -> Result<(), Box<dyn Error>>;
    fn handle_events(&self) -> Result<Event, Box<dyn Error>>;
}
