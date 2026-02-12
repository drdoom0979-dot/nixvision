use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub cameras: Vec<CameraSettings>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CameraSettings {
    pub name: String,
    pub url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = Path::new("config/settings.toml");
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        if !Path::new("config").exists() {
            fs::create_dir_all("config")?;
        }
        let content = toml::to_string(self).unwrap();
        fs::write("config/settings.toml", content)
    }

    // 1. Corregimos el default para que use la lista de cámaras
    pub fn default() -> Self {
        Self {
            cameras: vec![CameraSettings {
                name: "Webcam Local".to_string(),
                url: "0".to_string(),
            }],
        }
    }


}