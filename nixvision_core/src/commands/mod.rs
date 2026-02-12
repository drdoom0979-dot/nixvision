use inquire::Select;
use crate::ui::Interface;
use crate::io::frames::FrameCapture;
use crate::config::settings::{AppConfig,CameraSettings};
use crate::pipelines::processing::DynamicPipeline;


pub struct CommandManager;

impl CommandManager {
    pub fn run_interactive_menu() -> opencv::Result<()> {
        

        // 2. Menú de selección
        let opciones = vec!["Image", "Camera", "Salir"];
        let seleccion = Select::new("--- PANEL DE CONTROL NIXVISION ---", opciones).prompt();

        // 3. Match de opciones (Quitamos el uso de &gui)
        match seleccion {
            Ok("Image") => {
                let _ = Self::handle_img();
            }
            Ok("Camera") => {
                Self::handle_camera_menu()?;
            }
            Ok("Salir") => println!("Saliendo de NixVision..."),
            _ => println!("Operación cancelada."),
        }

        Ok(())
    }

    fn handle_img() -> opencv::Result<()> {
        // 1. Entrada de la imagen original
        let input_path = Interface::ask_text("> Ruta de la imagen de origen:", "fruta.jpg");
        let img = FrameCapture::load_image(&input_path)?;

        // 2. Configurar la "receta" del pipeline (Pide parámetros solo si se eligen)
        let receta = Self::construir_pipeline_usuario();

        // 3. Procesar la imagen con los parámetros dinámicos
        Interface::info("Procesando imagen con NixVision Core...");
        let res = DynamicPipeline::process(&img, &receta)?;

        // 4. Nombre de salida (Se guarda en la carpeta actual donde ejecutas el CLI)
        let out_name = Interface::ask_text("> Nombre del archivo de salida:", "resultado_procesado.jpg");
        
        Interface::info("Exportando y aplicando filtros...");
        for i in 1..=50 {
            Interface::progress_bar(i, 50);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        println!(); 

        // Guarda directamente en la ruta de ejecución
        FrameCapture::save_image(&res, &out_name)?;
        Interface::success(&format!("Imagen guardada correctamente como: {}", out_name));
        
        Ok(())
    }


    fn handle_capture_frame(cam: &crate::config::settings::CameraSettings) -> opencv::Result<()> {
        // 1. Configurar la "receta" del pipeline
        let receta: Vec<(i32, i32, f64, f64)> = Self::construir_pipeline_usuario();

        // 2. Parámetros de captura
        let fps: f64 = Interface::ask_text("FPS de captura:", "10.0").parse().unwrap_or(10.0);
        let segundos: u64 = Interface::ask_text("Segundos de duración:", "2").parse().unwrap_or(2);

        // 🔥 3. Gestión de Carpeta Dinámica
        // Primero preguntamos por el nombre de la carpeta raíz (ej. "Practica2" o "Pruebas_Lunes")
        let mut session_name = Interface::ask_text("Nombre de la carpeta principal:", "Capturas");
        
        // Construimos la ruta: nombre_sesion/nombre_camara
        let mut folder = format!("{}/{}", session_name.replace(" ", "_"), cam.name.replace(" ", "_"));

        // Validación para no sobrescribir sesiones anteriores
        while std::path::Path::new(&folder).exists() {
            Interface::info(&format!("⚠️ La ruta '{}' ya existe.", folder));
            let nuevo_nombre = Interface::ask_text("Escribe un nombre de sesión diferente:", &format!("{}_v2", session_name));
            session_name = nuevo_nombre;
            folder = format!("{}/{}", session_name.replace(" ", "_"), cam.name.replace(" ", "_"));
        }

        // Crear la estructura de directorios
        std::fs::create_dir_all(&folder).unwrap_or_default();

        Interface::info(&format!("🚀 Iniciando captura en {}...", cam.name));

        // 4. Obtener frames (Paso de Adquisición) [cite: 8]
        let frames = FrameCapture::capture_sequence(&cam.url, fps, segundos)?;

        // 5. Procesar y guardar cada imagen dentro de la carpeta seleccionada
        for (i, frame) in frames.iter().enumerate() {
            // El pipeline procesa el frame según la receta elegida [cite: 5]
            let procesada = DynamicPipeline::process(frame, &receta)?; 
            
            // Formato: nombre_sesion/nombre_camara/frame_001.jpg
            let filename = format!("{}/frame_{:03}.jpg", folder, i);
            FrameCapture::save_image(&procesada, &filename)?;
            Interface::progress_bar(i + 1, frames.len());
        }

        Interface::success(&format!("\nProceso completado. Archivos en: {}", folder));
        Ok(())
    }


    fn construir_pipeline_usuario() -> Vec<(i32, i32, f64, f64)> {
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
                    if let Ok(o) = opt { 
                        pipeline.push((1, if o == "Grises" { 1 } else { 2 }, 0.0, 0.0)); 
                    }
                }
                Ok("2. Iluminación") => {
                    let opt = Select::new("Método:", vec!["Normalizar", "Background Correction", "CLAHE"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Normalizar" => 1, "Background Correction" => 2, _ => 3 };
                        let mut p1 = 0.0;
                        if val == 3 {
                            p1 = Interface::ask_text("Clip Limit para CLAHE:", "2.0").parse().unwrap_or(2.0);
                        }
                        pipeline.push((2, val, p1, 0.0)); 
                    }
                }
                Ok("3. Ruido") => {
                    let opt = Select::new("Filtro:", vec!["Gaussian Blur", "Median Blur", "Bilateral Filter"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Gaussian Blur" => 1, "Median Blur" => 2, _ => 3 };
                        // Preguntamos el tamaño del kernel (Punto 2 y 31 de la práctica) [cite: 19, 31]
                        let size = Interface::ask_text("Tamaño del Blur (impar):", "5").parse().unwrap_or(5.0);
                        pipeline.push((3, val, size, 0.0)); 
                    }
                }
                Ok("4. Bordes/Canny") => {
                    let opt = Select::new("Algoritmo:", vec!["Sharpen", "Laplacian", "Canny (Bordes)"]).prompt();
                    if let Ok(o) = opt { 
                        let val = match o { "Sharpen" => 1, "Laplacian" => 2, _ => 3 };
                        let mut p1 = 0.0;
                        let mut p2 = 0.0;
                        if val == 3 {
                            // Parámetros dinámicos para el reporte (Punto 20 y 31) 
                            p1 = Interface::ask_text("Canny Low Threshold:", "50.0").parse().unwrap_or(50.0);
                            p2 = Interface::ask_text("Canny High Threshold:", "150.0").parse().unwrap_or(150.0);
                        }
                        pipeline.push((4, val, p1, p2)); 
                    }
                }
                Ok("5. Geometría (Afín)") => {
                    let opt = Select::new("Transformación:", vec!["Traslación", "Rotación"]).prompt();
                    if let Ok(o) = opt { 
                        let (val, p1, p2) = if o == "Traslación" {
                            let x = Interface::ask_text("X:", "50.0").parse().unwrap_or(50.0);
                            let y = Interface::ask_text("Y:", "50.0").parse().unwrap_or(50.0);
                            (1, x, y)
                        } else {
                            let ang = Interface::ask_text("Ángulo:", "45.0").parse().unwrap_or(45.0);
                            (2, ang, 0.0)
                        };
                        pipeline.push((5, val, p1, p2)); 
                    }
                }
                Ok("6. Contornos (Métricas)") => {
                    // Filtro de área mínima para evitar ruido (Punto 26) [cite: 26]
                    let area = Interface::ask_text("Área mínima para filtrar:", "500.0").parse().unwrap_or(500.0);
                    Interface::info("Se ha añadido la extracción de Área y Perímetro.");
                    pipeline.push((6, 1, area, 0.0)); 
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
        // 1. Cargar la configuración actual desde la ruta global ($HOME/.nixvision)
        let mut config = AppConfig::load(); 
        
        // 2. Crear el vector de opciones para el menú
        let mut opciones: Vec<String> = config.cameras
            .iter()
            .map(|c| format!("📷 {}", c.name))
            .collect();
        
        // 3. Añadir opciones administrativas y de salida
        opciones.push("➕ New Camera".to_string());
        opciones.push("🗑️ Remove Camera".to_string()); // Nueva opción
        opciones.push("🚪 Salir".to_string());

        let seleccion = Select::new("--- Camera Selection ---", opciones).prompt();

        match seleccion {
            Ok(choice) if choice == "➕ New Camera" => {
                Interface::info("Configurando nueva cámara...");
                let camera_name = Interface::ask_text("Nombre de la cámara:", "Camara_Dahua");
                let camera_url = Interface::ask_text("URL o Índice:", "0");

                let new_camera = CameraSettings {
                    name: camera_name,
                    url: camera_url,
                };

                config.cameras.push(new_camera);

                if let Err(e) = config.save() {
                    Interface::error(&format!("Error al guardar: {}", e));
                } else {
                    Interface::success("Cámara guardada correctamente.");
                }
            }

            // 🔥 LÓGICA PARA BORRAR CÁMARAS
            Ok(choice) if choice == "🗑️ Remove Camera" => {
                let cam_names: Vec<String> = config.cameras
                    .iter()
                    .filter(|c| c.name != "Webcam Local") // Excluimos la protegida
                    .map(|c| c.name.clone())
                    .collect();
                
                if cam_names.is_empty() {
                    Interface::info("No hay cámaras para eliminar.");
                } else {
                    let to_remove = Select::new("Selecciona la cámara a eliminar:", cam_names).prompt();
                    
                    if let Ok(name) = to_remove {
                        if config.remove_camera(&name) {
                            let _ = config.save(); // Persistir el cambio en el archivo global
                            Interface::success(&format!("Cámara '{}' eliminada exitosamente.", name));
                        }
                    }
                }
            }

            Ok(choice) if choice == "🚪 Salir" => {
                println!("Regresando...");
            }

            Ok(choice) => {
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