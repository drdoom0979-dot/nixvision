use inquire::Select;
use crate::ui::Interface;
use crate::io::frames::FrameCapture;
use nixvision_lib::filters::affine::NixAffine;
use crate::config::settings::{AppConfig,CameraSettings};
use crate::pipelines::processing::DynamicPipeline;


pub struct CommandManager;

impl CommandManager {
    pub fn run_interactive_menu() -> opencv::Result<()> {
        

        // 2. Menú de selección
        let opciones = vec!["Traslación", "Rotación", "Camera", "Salir"];
        let seleccion = Select::new("--- PANEL DE CONTROL NIXVISION ---", opciones).prompt();

        // 3. Match de opciones (Quitamos el uso de &gui)
        match seleccion {
            Ok("Traslación") => {
                let _ = Self::handle_translation();
            }
            Ok("Rotación") => {
                let _ = Self::handle_rotation();
            }
            Ok("Camera") => {
                Self::handle_camera_menu()?;
            }
            Ok("Salir") => println!("Saliendo de NixVision..."),
            _ => println!("Operación cancelada."),
        }

        Ok(())
    }

    fn handle_translation() -> opencv::Result<()> {
        // 1. Entrada de ruta
        let input_path = Interface::ask_text("> Ruta de la imagen:", "Ave_1.jpg");

        let img = FrameCapture::load_image(&input_path)?;
        
        let tx: f32 = Interface::ask_text("> Ingrese desplazamiento en X:", "0.0").parse().unwrap_or(0.0);
        let ty: f32 = Interface::ask_text("> Ingrese desplazamiento en Y:", "0.0").parse().unwrap_or(0.0);
        let out_name = Interface::ask_text("> Guardar resultado como:", "traslacion_res.jpg");

        Interface::info("Calculando matriz de traslación afín...");
        let res = NixAffine::translate(&img, tx, ty)?;

        Interface::info(&format!("Exportando datos a {}...", out_name));
        for i in 1..=50 {
            Interface::progress_bar(i, 50);
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
        println!(); 

        FrameCapture::save_image(&res, &out_name)?;
        Interface::success(&format!("Imagen guardada como: {}", out_name));
        
        Ok(())
    }

    fn handle_rotation() -> opencv::Result<()> {
        let input_path = Interface::ask_text("> Ruta de la imagen:", "Ave_1.jpg");
        let img = FrameCapture::load_image(&input_path)?;

        let angle: f64 = Interface::ask_text("> Ingrese ángulo de rotación:", "0.0").parse().unwrap_or(0.0);
        let output_name = Interface::ask_text("> Archivo de salida:", "rotacion_res.jpg");

        let affine_engine = NixAffine::new(&img, 1.0)?;

        Interface::info(&format!("Preparando rotación de {}°...", angle));

        for i in 1..=50 {
            Interface::progress_bar(i, 50);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        println!(); 

        let res = affine_engine.rotate(&img, angle)?;
        
        FrameCapture::save_image(&res, &output_name)?;
        Interface::success(&format!("Proceso completado: {}", output_name));
        
        Ok(())
    }

    fn handle_capture_frame(cam: &crate::config::settings::CameraSettings) -> opencv::Result<()> {
        // 1. Configurar la "receta" del pipeline
        let receta = Self::construir_pipeline_usuario();
        
        if receta.is_empty() {
            return Ok(());
        }

        // 2. Parámetros de captura
        let fps: f64 = Interface::ask_text("FPS de captura:", "10.0").parse().unwrap_or(10.0);
        let segundos: u64 = Interface::ask_text("Segundos de duración:", "3").parse().unwrap_or(3);

        // 3. Preparar carpeta de salida
        let folder = format!("capturas/{}", cam.name.replace(" ", "_"));
        std::fs::create_dir_all(&folder).unwrap_or_default();

        Interface::info(&format!("Capturando en {}...", cam.name));

        // 4. Obtener frames secuenciales
        let frames = FrameCapture::capture_sequence(&cam.url, fps, segundos)?;

        // 5. Procesar con nixvision_lib y guardar
        for (i, frame) in frames.iter().enumerate() {
            let procesada = DynamicPipeline::process(frame, &receta)?; // Aplica tu lógica de procesamiento
            let filename = format!("{}/frame_{:03}.jpg", folder, i);
            
            FrameCapture::save_image(&procesada, &filename)?;
            Interface::progress_bar(i + 1, frames.len());
        }

        Interface::success(&format!("\nSecuencia guardada en: {}", folder));
        Ok(())
    }

    fn construir_pipeline_usuario() -> Vec<(i32, i32)> {
        let mut pipeline = Vec::new();
        let categorias = vec![
            "1. Color", 
            "2. Iluminación", 
            "3. Ruido", 
            "4. Bordes/Canny", 
            "5. Geometría (Afín)", 
            "6. Contornos (Métricas)",
            "🚀 Finalizar y Procesar"
        ];

        loop {
            let seleccion = Select::new("Añadir paso al pipeline:", categorias.clone()).prompt();
            
            match seleccion {
                Ok("1. Color") => {
                    let opt = Select::new("Conversión:", vec!["Grises", "HSV"]).prompt();
                    if let Ok(o) = opt { pipeline.push((1, if o == "Grises" { 1 } else { 2 })); }
                }
                Ok("2. Iluminación") => {
                    let opt = Select::new("Método:", vec!["Normalizar", "Background Correction", "CLAHE"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Normalizar" => 1, "Background Correction" => 2, _ => 3 };
                        pipeline.push((2, val)); 
                    }
                }
                Ok("3. Ruido") => {
                    let opt = Select::new("Filtro:", vec!["Gaussian Blur", "Median Blur", "Bilateral Filter"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Gaussian Blur" => 1, "Median Blur" => 2, _ => 3 };
                        pipeline.push((3, val)); 
                    }
                }
                Ok("4. Bordes/Canny") => {
                    let opt = Select::new("Algoritmo:", vec!["Sharpen", "Laplacian", "Canny (Bordes)"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Sharpen" => 1, "Laplacian" => 2, _ => 3 };
                        pipeline.push((4, val)); 
                    }
                }
                Ok("5. Geometría (Afín)") => {
                    let opt = Select::new("Transformación:", vec!["Traslación (50,50)", "Rotación (45°/Centro)"]).prompt();
                    if let Ok(o) = opt { 
                        let val = if o.contains("Traslación") { 1 } else { 2 };
                        pipeline.push((5, val)); 
                    }
                }
                Ok("6. Contornos (Métricas)") => {
                    Interface::info("Se ha añadido la extracción de Área y Perímetro.");
                    pipeline.push((6, 0)); // Paso final de análisis de la Práctica 2
                }
                Ok("🚀 Finalizar y Procesar") => break,
                _ => break,
            }
        }
        pipeline
    }

    

    fn handle_camera_actions(cam: &crate::config::settings::CameraSettings) -> opencv::Result<()> {
        let opciones = vec!["Frame Capture", "Rotación", "Real-time Vision", "Salir"];
        let titulo = format!("--- CONTROL: {} ---", cam.name);
        let seleccion = Select::new(&titulo, opciones).prompt();

        match seleccion {
            Ok("Frame Capture") => {
                Self::handle_capture_frame(cam)?;
            }
            
            Ok("Real-time Vision") => {
                Interface::info("Iniciando flujo RTSP...");
            }
            Ok("Salir") => println!("Saliendo de NixVision..."),
            _ => println!("Operación cancelada."),
        }
        Ok(())
    }


    fn handle_camera_menu() -> opencv::Result<()> {
        // 1. Cargar la configuración actual [cite: 7]
        let mut config = AppConfig::load(); 
        
        // 2. Crear el vector de opciones para el menú
        let mut opciones: Vec<String> = config.cameras
            .iter()
            .map(|c| format!("📷 {}", c.name)) // Usar un for implícito (map) para los nombres [cite: 17]
            .collect();
        
        // 3. Añadir opciones fijas
        opciones.push("➕ New Camera".to_string());
        opciones.push("🚪 Salir".to_string());

        let seleccion = Select::new("--- Camera Selection ---", opciones).prompt();

        match seleccion {
            Ok(choice) if choice == "➕ New Camera" => {
                Interface::info("Configurando nueva cámara...");

                let camera_name = Interface::ask_text(
                    "Ingresa el nombre de tu camara: ",
                    "Camara"
                );

                let camera_url = Interface::ask_text(
                    "Ingresa tu Url: ",
                    "rtsp://"
                );

                // 🔥 Crear nueva cámara
                let new_camera = CameraSettings {
                    name: camera_name,
                    url: camera_url,
                };

                // 🔥 Agregar al vector
                config.cameras.push(new_camera);

                // 🔥 Guardar
                if let Err(e) = config.save() {
                    eprintln!("Error al guardar configuración: {}", e);
                } else {
                    Interface::info("Cámara guardada correctamente.");
                }
            }

            Ok(choice) if choice == "🚪 Salir" => {
                println!("Saliendo de NixVision...");
            }
            Ok(choice) => {
                // 4. Buscar la cámara seleccionada en el vector original
                let cam_name = choice.replace("📷 ", "");
                if let Some(cam) = config.cameras.iter().find(|c| c.name == cam_name) {
                    Self::handle_camera_actions(cam)?;
                    
                }
            }
            _ => println!("Operación cancelada."),
        }
        Ok(())
    }
}