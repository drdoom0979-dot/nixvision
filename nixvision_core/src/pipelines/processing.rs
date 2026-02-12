use nixvision_lib::filters::{
    affine::NixAffine, color::ColorConverter, contour::NixContour,
    edges::EdgeManager, illumination::IlluminationManager, noise::NoiseReducer
};
use opencv::{prelude::*, core, Result};

pub struct DynamicPipeline;

impl DynamicPipeline {
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
                    let mut display = current_mat.clone();
                    
                    // El área mínima se recibe desde p1 para filtrar ruido (Punto 6 [cite: 26])
                    let min_area = *p1; 

                    // Obtenemos métricas: Área, Perímetro y Bounding Box [cite: 23, 24, 25]
                    let metrics = NixContour::find_and_measure(&current_mat, &mut display, min_area)?;

                    // Resaltar el contorno más grande (Punto 7 de la práctica )
                    if let Some(largest) = metrics.iter().max_by(|a, b| a.area.partial_cmp(&b.area).unwrap()) {
                        NixContour::draw_highlight(&mut display, largest)?;
                        
                        // Imprimimos los datos necesarios para tu TABLA DE 5 FRUTAS 
                        println!("✅ Fruta Detectada -> Área: {:.2}, Perímetro: {:.2}", largest.area, largest.perimeter);
                    }

                    display 
                },
                _ => current_mat,
            };
        }
        Ok(current_mat)
    }
}