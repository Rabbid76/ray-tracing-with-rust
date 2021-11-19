use crate::image;
use crate::serialization::IdConstructor;
use ray_tracing_core::texture;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BitmapFile {
    pub id: IdConstructor,
    pub filename: String,
}

impl BitmapFile {
    pub fn file_to_texture(filename: &String) -> Result<texture::BitmapTexture, Box<dyn Error>> {
        let (nx, ny, pixel_data) = match image::load_image(&filename) {
            Ok((nx, ny, pixel_data)) => (nx, ny, pixel_data),
            Err(e) => {
                eprintln!("error reading file: {}", filename);
                return Err(e);
            }
        };
        Ok(texture::BitmapTexture::new(nx, ny, pixel_data))
    }

    pub fn to_texture(
        &self,
        index: usize,
        root_path: &Option<String>,
    ) -> Result<texture::BitmapTexture, Box<dyn Error>> {
        let filename = match root_path {
            Some(root_path) => {
                let path = Path::new(&self.filename);
                if !path.is_absolute() {
                    String::from(Path::new(&root_path).join(path).to_str().unwrap())
                } else {
                    self.filename.clone()
                }
            }
            None => self.filename.clone(),
        };
        let (nx, ny, pixel_data) = match image::load_image(&filename) {
            Ok((nx, ny, pixel_data)) => (nx, ny, pixel_data),
            Err(e) => {
                eprintln!("error reading file: {}", filename);
                return Err(e);
            }
        };
        Ok(texture::BitmapTexture::new_id(
            self.id.get_id(index),
            nx,
            ny,
            pixel_data,
        ))
    }
}

#[cfg(test)]
mod bitmap_file_test {
    use super::*;

    #[test]
    fn bitmap_file_to_texture() {
        let bt = BitmapFile {
            id: IdConstructor::Single(2),
            filename: "../resource/texture/physical-free-world-map-b1.jpg".to_string(),
        };
        let t = match bt.to_texture(0, &None) {
            Ok(t) => t,
            Err(e) => panic!("read file error {}", e),
        };
        assert_eq!(t.nx, 1000);
        assert_eq!(t.ny, 500);
        assert_eq!(t.data.len(), t.nx * t.ny * 4);
    }
}
