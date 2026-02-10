use inquire::Select;
use crate::ui::Interface;
use crate::io::frames::FrameCapture;
use nixvision_lib::filters::affine::NixAffine;

pub struct CommandManager;

impl CommandManager {
    pub fn run_interactive_menu() -> opencv::Result<()> {
        

        // 2. Menú de selección
        let opciones = vec!["Traslación", "Rotación", "Real-time Vision (RTSP)", "Salir"];
        let seleccion = Select::new("--- PANEL DE CONTROL NIXVISION ---", opciones).prompt();

        // 3. Match de opciones (Quitamos el uso de &gui)
        match seleccion {
            Ok("Traslación") => {
                let _ = Self::handle_translation();
            }
            Ok("Rotación") => {
                let _ = Self::handle_rotation();
            }
            Ok("Real-time Vision (RTSP)") => {
                Interface::info("Iniciando flujo RTSP...");
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
}