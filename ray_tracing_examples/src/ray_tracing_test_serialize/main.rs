use ray_tracing_core::test::TestSceneSimple;
use ray_tracing_utility::serialization::json;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    let scene = TestSceneSimple::new().scene;
    let json_string = json::serialize_scene(&scene)?;
    print!("{}", json_string);
    fs::write("scene/TestSceneSimple.json", json_string)?;
    Ok(())
}
