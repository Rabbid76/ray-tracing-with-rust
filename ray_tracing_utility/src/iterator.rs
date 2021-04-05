mod iterator_exp2;
pub use self::iterator_exp2::IteratorExp2;
use std::sync::{Arc, Mutex};

pub trait ViewportIterator: Iterator<Item = (usize, usize, usize)> + Sync + Send {
    fn create_new(&self) -> Arc<Mutex<dyn ViewportIterator>>;
}
