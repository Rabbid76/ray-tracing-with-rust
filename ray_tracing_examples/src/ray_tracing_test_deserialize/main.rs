use ray_tracing_utility::serialization::json;
use std::error::Error;

static TEST_SCENE_STR: &str = r#"{
    "configuration_id": 14,
    "camera_id": 12,
    "sky_id": 13,
    "root_node_id": 7,
    "objects": [
      { "ConstantTexture": { "id": 1, "color": [0.5, 0.1, 0.1] } },
      { "Lambertian": { "id": 2, "albedo": 1 } },
      { "Sphere": { "id": 3, "center": [0.0, 0.0, -1.0], "radius": 0.5, "material": 2 } },
      { "ConstantTexture": { "id": 4, "color": [0.1, 0.1, 0.1] } },
      { "Lambertian": { "id": 5, "albedo": 4 } },
      { "Sphere": { "id": 6, "center": [0.0, -100.5, -1.0], "radius": 100.0, "material": 5 } },
      { "Collection": { "id": 7, "object_id_list": [6, 3] } },
      { "Camera": {
          "id": 12,
          "lower_left_corner": [-2.0, -1.0, -1.0],
          "horizontal": [4.0, 0.0, 0.0],
          "vertical": [0.0, 2.0, 0.0],
          "origin": [0.0, 0.0, 0.0],
          "lense_radius": 0.0,
          "time_from": 0.0,
          "time_to": 0.0
        }
      },
      { "Sky": { "id": 13, "base_color": [0.5, 0.7, 1.0] } },
      { "Configuration": { "id": 14, "maximum_depth": 50 } }
    ]
  }"#;

fn main() -> Result<(), Box<dyn Error>> {
    let scene = json::deserialize_scene(TEST_SCENE_STR)?;
    print!("{}", scene.configuration.maximum_depth);
    Ok(())
}
