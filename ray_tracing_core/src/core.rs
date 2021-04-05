pub mod object;

mod hit_record;
pub use self::hit_record::HitRecord;

mod scatter_record;
pub use self::scatter_record::ScatterRecord;

mod configuration;
pub use self::configuration::Configuration;

mod scene;
pub use self::scene::Scene;

mod camera;
pub use self::camera::Camera;
