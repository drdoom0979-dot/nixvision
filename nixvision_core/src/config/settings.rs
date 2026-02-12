use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{PathBuf};

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
    /// Obtiene la ruta global en el HOME del usuario
    fn get_config_path() -> PathBuf {
        let mut path = std::env::var_os("HOME")
            .map(PathBuf::from)
            .expect("No se pudo encontrar la carpeta HOME");
        
        path.push(".nixvision");
        path.push("settings.toml");
        path
    }

    fn get_config_dir() -> PathBuf {
        let mut path = std::env::var_os("HOME")
            .map(PathBuf::from)
            .expect("No se pudo encontrar la carpeta HOME");
        path.push(".nixvision");
        path
    }

    /// Carga la configuración desde el archivo TOML global
    pub fn load() -> Self {
        let path = Self::get_config_path();
        
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_else(|_| {
                eprintln!("⚠️ Error al parsear TOML en {:?}, cargando default...", path);
                Self::default()
            })
        } else {
            let config = Self::default();
            let _ = config.save().ok(); 
            config
        }
    }

    /// Guarda los cambios de forma persistente
    pub fn save(&self) -> std::io::Result<()> {
        let dir = Self::get_config_dir();
        let file = Self::get_config_path();

        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

        fs::write(&file, content)
    }

    /// 🔥 NUEVA FUNCIÓN: Borra una cámara por su nombre
    /// Borra una cámara por su nombre, protegiendo la cámara por defecto
    pub fn remove_camera(&mut self, name: &str) -> bool {
        // Bloqueamos el borrado si es la cámara local 
        if name == "Webcam Local" {
            eprintln!("🛑 No se puede eliminar la cámara predeterminada del sistema.");
            return false;
        }

        let initial_len = self.cameras.len();
        self.cameras.retain(|c| c.name != name);
        
        self.cameras.len() < initial_len
    }

    pub fn default() -> Self {
        Self {
            cameras: vec![CameraSettings {
                name: "Webcam Local".to_string(),
                url: "0".to_string(),
            }],
        }
    }
}