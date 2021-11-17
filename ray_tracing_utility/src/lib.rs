//! # Crate ray_tracing_utility
//!
//! GitHub page [rabbid76.github.io/ray-tracing-with-rust](https://rabbid76.github.io/ray-tracing-with-rust/)  
//! GitHub repository [Rabbid76/ray-tracing-with-rust](https://github.com/Rabbid76/ray-tracing-with-rust)
//!
//! Deserialization and rendering process
//!
//! # Example
//!
//! ## Process
//!
//! ```rust
//! use ray_tracing_core::random;
//! use ray_tracing_core::test::TestSceneSimple;
//! use ray_tracing_core::types::ColorRGB;
//! use ray_tracing_core::types::FSize;
//! use ray_tracing_utility::iterator::IteratorExp2;
//!
//! fn main() {
//!     let cx = 40;
//!     let cy = 20;
//!     let samples = 10;
//!     let scene = TestSceneSimple::new().scene;
//!     let ray_trace_iter = IteratorExp2::new(cx, cy);
//!
//!     let mut pixel_data: Vec<u8> = Vec::with_capacity(cx * cy * 4);
//!     pixel_data.resize(cx * cy * 4, 0);
//!
//!     for (x, y, _size) in ray_trace_iter {
//!         let mut c = ColorRGB::new(0.0, 0.0, 0.0);
//!         for _ in 0..samples {
//!             let u = (x as FSize + random::generate_size()) / cx as FSize;
//!             let v = 1.0 - (y as FSize + random::generate_size()) / cy as FSize;
//!             c = c + scene.ray_trace_color(u, v);
//!         }
//!         c = c / samples as FSize;
//!
//!         let i = (y * cx) + x;
//!         pixel_data[i * 4] = (c[0].sqrt() * 255.0).round() as u8;
//!         pixel_data[i * 4 + 1] = (c[1].sqrt() * 255.0).round() as u8;
//!         pixel_data[i * 4 + 2] = (c[2].sqrt() * 255.0).round() as u8;
//!         pixel_data[i * 4 + 3] = 255;
//!     }
//!
//!     // [...]
//! }
//! ```
//!
//! ## Serialization
//!
//! ```rust
//! use ray_tracing_core::test::TestSceneSimple;
//! use ray_tracing_utility::serialization::json;
//! use std::error::Error;
//! use std::fs;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let scene = TestSceneSimple::new().scene;
//!     let json_string = json::serialize_scene(&scene)?;
//!     print!("{}", json_string);
//!     fs::write("TestSceneSimple.json", json_string)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Deserialization
//!
//! ```rust
//! use ray_tracing_utility::serialization::json;
//! use std::error::Error;
//!
//! static TEST_SCENE_STR: &str = r#"{
//!     "configuration_id": 14,
//!     "camera_id": 12,
//!     "sky_id": 13,
//!     "root_node_id": 7,
//!     "objects": [
//!       { "ConstantTexture": { "id": 1, "color": [0.5, 0.1, 0.1] } },
//!       { "Lambertian": { "id": 2, "albedo": 1 } },
//!       { "Sphere": { "id": 3, "center": [0.0, 0.0, -1.0], "radius": 0.5, "material": 2 } },
//!       { "ConstantTexture": { "id": 4, "color": [0.1, 0.1, 0.1] } },
//!       { "Lambertian": { "id": 5, "albedo": 4 } },
//!       { "Sphere": { "id": 6, "center": [0.0, -100.5, -1.0], "radius": 100.0, "material": 5 } },
//!       { "Collection": { "id": 7, "object_id_list": [6, 3] } },
//!       { "Camera": {
//!           "id": 12,
//!           "lower_left_corner": [-2.0, -1.0, -1.0],
//!           "horizontal": [4.0, 0.0, 0.0],
//!           "vertical": [0.0, 2.0, 0.0],
//!           "origin": [0.0, 0.0, 0.0],
//!           "lense_radius": 0.0,
//!           "time_from": 0.0,
//!           "time_to": 0.0
//!         }
//!       },
//!       { "Sky": { "id": 13, "nadir_color": [1.0, 1.0, 1.0], "zenith_color": [0.5, 0.7, 1.0] } },
//!       { "Configuration": { "id": 14, "maximum_depth": 50 } }
//!     ]
//!   }"#;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let scene = json::deserialize_scene(TEST_SCENE_STR)?;
//!     print!("{}", scene.configuration.maximum_depth);
//!     Ok(())
//! }
//!```

pub mod image;
pub mod iterator;
pub mod serialization;
pub mod thread;
pub mod view;
