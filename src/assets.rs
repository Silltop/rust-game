use macroquad::prelude::*;
use std::collections::HashMap;
use std::fs;

pub async fn load_assets() -> HashMap<String, Texture2D> {
    let mut assets = HashMap::new();
    let paths = match fs::read_dir("assets") {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Failed to read assets directory: {}", e);
            return assets;
        }
    };

    for path in paths {
        let path = match path {
            Ok(path) => path.path(),
            Err(e) => {
                eprintln!("Failed to read path: {}", e);
                continue;
            }
        };

        if path.is_file() {
            let file_name = match path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name.to_string(),
                None => {
                    eprintln!("Failed to get file name for path: {:?}", path);
                    continue;
                }
            };

            let texture = match load_texture(path.to_str().unwrap()).await {
                Ok(texture) => texture,
                Err(e) => {
                    eprintln!(
                        "Failed to load texture: {:?}, error: {}",
                        path.to_str().unwrap(),
                        e
                    );
                    continue;
                }
            };

            if texture.width() == 0.0 || texture.height() == 0.0 {
                eprintln!("Unsupported or broken image: {:?}", path.to_str().unwrap());
                continue;
            }

            assets.insert(file_name, texture);
        }
    }

    return assets;
}
