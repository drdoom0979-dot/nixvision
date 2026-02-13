use nixvision_lib::filters::{
    affine::NixAffine, color::ColorConverter, contour::NixContour,
    edges::EdgeManager, illumination::IlluminationManager, noise::NoiseReducer
};
use opencv::{prelude::*, core,imgproc, Result};

pub struct ProcessResult {
    pub image: opencv::core::Mat,
    pub area: f64,
    pub perimeter: f64, 
    pub width: i32,     
    pub height: i32,    
    pub detected: bool,
}
pub struct DynamicPipeline;

impl DynamicPipeline {

    pub fn process_with_metadata(
        img: &Mat, 
        receta: &Vec<(i32, i32, f64, f64)>
    ) -> Result<ProcessResult> {
        let mut current_mat = img.clone();

        for (step_type, option, p1, p2) in receta {
            match *step_type {
                6 => { // Paso de Contornos (Puntos 4-7)
                    let mut display = Mat::default();
                    if current_mat.channels() == 1 {
                        imgproc::cvt_color_def(&current_mat, &mut display, imgproc::COLOR_GRAY2BGR)?;
                    } else {
                        display = current_mat.clone();
                    }

                    // Ejecutamos medición real con tus filtros
                    let metrics = NixContour::find_and_measure(&current_mat, &mut display, *p1)?;

                    if let Some(largest) = metrics.iter().max_by(|a, b| a.area.partial_cmp(&b.area).unwrap()) {
                        NixContour::draw_highlight(&mut display, largest)?;
                        
                        // Retorno inmediato: Aquí las variables no necesitan ser mutables 
                        // porque se pasan directamente al inicializador.
                        return Ok(ProcessResult {
                            image: display,
                            area: largest.area,
                            perimeter: largest.perimeter, 
                            width: largest.width,         
                            height: largest.height,       
                            detected: true,
                        });
                    }
                },
                _ => {
                    current_mat = Self::apply_step(&current_mat, (*step_type, *option, *p1, *p2))?;
                }
            }
        }

        // Si llegamos aquí, no hubo detección. Retornamos valores por defecto (Punto 5)
        Ok(ProcessResult {
            image: current_mat,
            area: 0.0,
            perimeter: 0.0,
            width: 0,
            height: 0,
            detected: false,
        })
    }

    /// Implementación modular de tus filtros
    pub fn apply_step(img: &Mat, step: (i32, i32, f64, f64)) -> Result<Mat> {
        let (step_type, option, p1, p2) = step;
        match step_type {
            1 => match option { // Color
                1 => ColorConverter::to_grayscale(img),
                2 => ColorConverter::to_hsv(img),
                _ => Ok(img.clone()),
            },
            2 => match option { // Iluminación
                1 => IlluminationManager::normalize(img),
                3 => IlluminationManager::apply_clahe(img, p1, core::Size::new(8, 8)),
                _ => Ok(img.clone()),
            },
            3 => { // Ruido
                let kernel = p1 as i32;
                match option {
                    1 => NoiseReducer::gaussian(img, kernel),
                    2 => NoiseReducer::median(img, kernel),
                    _ => Ok(img.clone()),
                }
            },
            4 => match option { // Bordes/Canny
                1 => EdgeManager::sharpen(img),
                3 => EdgeManager::canny(img, p1, p2),
                _ => Ok(img.clone()),
            },
            5 => match option { // Afín
                1 => NixAffine::translate(img, p1 as f32, p2 as f32),
                2 => {
                    let engine = NixAffine::new(img, 1.0)?;
                    engine.rotate(img, p1)
                },
                _ => Ok(img.clone()),
            },
            _ => Ok(img.clone()),
        }
    }
    /// Procesa una imagen aplicando parámetros dinámicos (tipo, opción, param1, param2)
    /// Esto permite ajustar Canny y Blur en tiempo real para el reporte[cite: 31].
    pub fn process(img: &Mat, steps: &[(i32, i32, f64, f64)]) -> Result<Mat> {
        let mut current_mat = img.clone();

        for (step_type, option, p1, p2) in steps {
            current_mat = match step_type {
                // 1. CONVERSIÓN DE COLOR (Punto 2 del procedimiento [cite: 19])
                1 => match option {
                    1 => ColorConverter::to_grayscale(&current_mat)?,
                    2 => ColorConverter::to_hsv(&current_mat)?,
                    _ => current_mat,
                },
                // 2. CORRECCIÓN DE ILUMINACIÓN
                2 => match option {
                    1 => IlluminationManager::normalize(&current_mat)?,
                    3 => IlluminationManager::apply_clahe(&current_mat, *p1, core::Size::new(8, 8))?,
                    _ => current_mat,
                },
                // 3. REDUCCIÓN DE RUIDO (Punto 2: Gaussian Blur [cite: 19])
                3 => {
                    let kernel_size = *p1 as i32; // Usamos el parámetro ingresado por el usuario
                    match option {
                        1 => NoiseReducer::gaussian(&current_mat, kernel_size)?,
                        2 => NoiseReducer::median(&current_mat, kernel_size)?,
                        _ => current_mat,
                    }
                },
                // 4. REALCE DE BORDES Y SEGMENTACIÓN (Punto 3: Canny [cite: 20])
                4 => match option {
                    1 => EdgeManager::sharpen(&current_mat)?,
                    3 => EdgeManager::canny(&current_mat, *p1, *p2)?, // p1=low, p2=high threshold
                    _ => current_mat,
                },
                // 5. TRANSFORMACIONES AFINES
                5 => match option {
                    1 => NixAffine::translate(&current_mat, *p1 as f32, *p2 as f32)?,
                    2 => {
                        let engine = NixAffine::new(&current_mat, 1.0)?;
                        engine.rotate(&current_mat, *p1 )?
                    },
                    _ => current_mat,
                },
                // 6. EXTRACCIÓN DE CONTORNOS (Puntos 4-7 del procedimiento [cite: 21, 22])
                6 => {
                    let mut display = Mat::default();
                    
                    // Verificamos si la imagen actual son Bordes (1 canal)
                    if current_mat.channels() == 1 {
                        // Usamos cvt_color_def para evitar el error de los 5 argumentos
                        // Esto convierte Grises a BGR para poder dibujar en ROJO [cite: 38]
                        imgproc::cvt_color_def(&current_mat, &mut display, imgproc::COLOR_GRAY2BGR)?;
                    } else {
                        display = current_mat.clone();
                    }

                    // Ejecutamos la medición y detección (Puntos 4-7 del procedimiento [cite: 21, 22])
                    let metrics = NixContour::find_and_measure(&current_mat, &mut display, *p1)?;

                    if let Some(largest) = metrics.iter().max_by(|a, b| a.area.partial_cmp(&b.area).unwrap()) {
                        // Resaltamos el objeto más grande (Punto 7 / Extensión [cite: 27]) 
                        NixContour::draw_highlight(&mut display, largest)?;
                        
                        // Imprimimos los datos para tu tabla del Punto 5 [cite: 22, 23, 24, 25]
                        println!("✅ Objeto Detectado:");
                        println!("   - Área: {:.2} px", largest.area);
                        println!("   - Perímetro: {:.2} px", largest.perimeter);
                        println!("   - Bounding Box: {}x{} px (Ancho x Alto)", largest.width, largest.height);
                    }
                    display 
                },
                _ => current_mat,
            };
        }
        Ok(current_mat)
    }
}