static mut NEXT_OBJECT_ID: usize = 1;

pub struct Object {}

impl Object {
    pub fn new_id() -> usize {
        unsafe {
            let id = NEXT_OBJECT_ID;
            NEXT_OBJECT_ID += 1;
            id
        }
    }
}
