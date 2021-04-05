//! # Crate ray_tracing_core
//! 
//! GitHub page [rabbid76.github.io/ray-tracing-with-rust](https://rabbid76.github.io/ray-tracing-with-rust/)  
//! GitHub repository [Rabbid76/ray-tracing-with-rust](https://github.com/Rabbid76/ray-tracing-with-rust)
//! 
//! [![](https://stackexchange.com/users/flair/7322082.png)](https://stackoverflow.com/users/5577765/rabbid76?tab=profile)
//! 
//! Based on [Peter Shirley's](https://research.nvidia.com/person/peter-shirley) books:
//!
//! - ["Ray Tracing in One Weekend (Ray Tracing Minibooks Book 1)"](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
//! - ["Ray Tracing: the Next Week (Ray Tracing Minibooks Book 2)"](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
//! - ["Ray Tracing: The Rest of Your Life (Ray Tracing Minibooks Book 3)"](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
//!
//! ![cover scene - ray tracing 3](https://raw.githubusercontent.com/Rabbid76/ray-tracing-with-rust/main/rendering/RoomGlassSphere_800x800_100000_samples.png)
//! 
//! “Note that I avoid most “modern features” of C++, but inheritance and operator overloading are too useful for ray tracers to pass on.”  
//! ― [Peter Shirley](https://research.nvidia.com/person/peter-shirley), [Ray Tracing in One Weekend](https://www.goodreads.com/book/show/28794030-ray-tracing-in-one-weekend)
//! 
//! # Example
//!
//! ```rust
//! use ray_tracing_core::random;
//! use ray_tracing_core::test::TestSceneSimple;
//! use ray_tracing_core::types::ColorRGB;
//! use ray_tracing_core::types::FSize;
//!
//! fn main() {
//!     let cx = 40;
//!     let cy = 20;
//!     let samples = 10;
//!     let scene = TestSceneSimple::new().scene;
//!
//!     let mut pixel_data: Vec<u8> = Vec::with_capacity(cx * cy * 4);
//!     pixel_data.resize(cx * cy * 4, 0);
//!
//!     for y in 0..cy {
//!         for x in 0..cx {
//!             let mut c = ColorRGB::new(0.0, 0.0, 0.0);
//!             for _ in 0..samples {
//!                 let u = (x as FSize + random::generate_size()) / cx as FSize;
//!                 let v = 1.0 - (y as FSize + random::generate_size()) / cy as FSize;
//!                 c = c + scene.ray_trace_color(u, v);
//!             }
//!             c = c / samples as FSize;
//!
//!             let i = (y * cx) + x;
//!             pixel_data[i * 4] = (c[0].sqrt() * 255.0).round() as u8;
//!             pixel_data[i * 4 + 1] = (c[1].sqrt() * 255.0).round() as u8;
//!             pixel_data[i * 4 + 2] = (c[2].sqrt() * 255.0).round() as u8;
//!             pixel_data[i * 4 + 3] = 255;
//!         }
//!     }
//!
//!     // [...]
//! }
//!  ```

/// Ray tracing data types
///
/// Implementation of the data types used for the ray tracing calculations using [Crate `glm`](https://docs.rs/glm/0.2.3/glm/index.html)
pub mod types;

/// Random data generator
///
/// Generators for random data like vectors and colors using [Crate `rand`](https://docs.rs/rand/0.8.3/rand/)
pub mod random;

/// Ray Trace Math
///
/// Ray trace math objects and equations  
pub mod math;

/// Texture objects
///
/// Implementation of ray tracing textures
pub mod texture;

/// Material objects
///
/// Implementation of ray tracing materials
pub mod material;

/// Hit able objects
///
/// Implementation of ray tracing hit ables
pub mod hit_able;

/// Environment
///
/// Implementation of environment like sky
pub mod environment;

/// Ray Trace core
///
/// Ray trace core objects  
pub mod core;

/// Probability Density Function
pub mod probability_density_function;

/// Internal module for test for integration tests
pub mod test;
