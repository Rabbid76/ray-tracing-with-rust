use crate::math::Ray;
use crate::types::ColorRGB;
use std::error::Error;

mod sky;
pub use self::sky::Sky;

pub trait Environment: Sync + Send {
    fn get_id(&self) -> usize;

    fn get(&self, ray: &Ray) -> ColorRGB;

    fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Box<dyn Error>>;
}

pub trait Visitor {
    fn visit_sky(&mut self, t: &Sky) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod test_visitor {
    use super::*;

    struct TestVisitor {
        pub count_environment: usize,
    }

    impl Visitor for TestVisitor {
        fn visit_sky(&mut self, _: &Sky) -> Result<(), Box<dyn Error>> {
            self.count_environment += 1;
            Ok(())
        }
    }

    #[test]
    pub fn test_visitor_environment() {
        let t = Sky::new(ColorRGB::new(0.0, 0.0, 0.0), ColorRGB::new(1.0, 1.0, 1.0));
        let mut v = TestVisitor {
            count_environment: 0,
        };
        t.accept(&mut v).unwrap();
        assert_eq!(v.count_environment, 1);
    }
}
