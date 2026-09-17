//! Exercise expanded import paths against the pinned model inventory.
use serde_json::{Value, json};
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries: Vec<Value> = serde_json::from_slice(&fs::read(
        std::env::args().nth(1).ok_or("manifest path required")?,
    )?)?;
    let mut results = Vec::new();
    for entry in entries {
        let outcome = (|| -> Result<Value, Box<dyn std::error::Error>> {
            if let Some(error) = entry["preparation_error"].as_str() {
                return Err(error.into());
            }
            let bytes = fs::read(entry["path"].as_str().ok_or("path")?)?;
            let read = |key: &str| -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
                entry[key]
                    .as_array()
                    .ok_or("array")?
                    .iter()
                    .map(|p| Ok(fs::read(p.as_str().ok_or("path")?)?))
                    .collect()
            };
            let prepared = three_rs_wasm::compression::prepare_gltf(&bytes, &read("buffers")?)?;
            let imported = three_rs_wasm::gltf::import_animated(
                &prepared.asset,
                &prepared.buffers,
                &read("images")?,
            )?;
            let mut scene = three_rs_wasm::scene::Scene::new();
            let instance = imported.instantiate(&mut scene)?;
            scene.update()?;
            for &mesh in &instance.meshes {
                three_rs_wasm::deformation::evaluate(&scene, mesh)?;
            }
            let mut mixer = three_rs_wasm::animation::AnimationMixer::default();
            for clip in &instance.clips {
                let index = mixer.play(clip.clone())?;
                mixer.update(&mut scene, 0.25)?;
                scene.update()?;
                for &mesh in &instance.meshes {
                    three_rs_wasm::deformation::evaluate(&scene, mesh)?;
                }
                mixer.restore(&mut scene)?;
                mixer.actions[index].enabled = false;
            }
            Ok(
                json!({"status":"imported-and-evaluated","meshes":instance.meshes.len(),"clips":instance.clips.len()}),
            )
        })();
        results.push(json!({"asset":entry["asset"],"sha256":entry["sha256"],"result":outcome.unwrap_or_else(|e|json!({"status":"rejected","error":e.to_string().chars().take(400).collect::<String>()}))}));
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
