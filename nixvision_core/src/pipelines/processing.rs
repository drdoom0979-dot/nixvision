use nixvision_lib::filters::{
    affine::NixAffine, color::ColorConverter, contour::NixContour,
    edges::EdgeManager, illumination::IlluminationManager, noise::NoiseReducer
};
use opencv::{prelude::*, core, Result};

pub struct DynamicPipeline;

impl DynamicPipeline {
    /// Procesa una imagen aplicando una "receta" de pasos seleccionables.
    pub fn process(img: &Mat, steps: &[(i32, i32)]) -> Result<Mat> {
        let mut current_mat = img.clone();

        for (step_type, option) in steps {
            current_mat = match step_type {
                // 1. CONVERSIÓN DE COLOR
                1 => match option {
                    1 => ColorConverter::to_grayscale(&current_mat)?,
                    2 => ColorConverter::to_hsv(&current_mat)?,
                    _ => current_mat,
                },
                // 2. CORRECCIÓN DE ILUMINACIÓN
                2 => match option {
                    1 => IlluminationManager::normalize(&current_mat)?,
                    2 => IlluminationManager::background_correction(&current_mat)?,
                    3 => IlluminationManager::apply_clahe(&current_mat, 2.0, core::Size::new(8, 8))?,
                    _ => current_mat,
                },
                // 3. REDUCCIÓN DE RUIDO
                3 => match option {
                    1 => NoiseReducer::gaussian(&current_mat, 5)?,
                    2 => NoiseReducer::median(&current_mat, 5)?,
                    3 => NoiseReducer::bilateral(&current_mat, 9, 75.0, 75.0)?,
                    _ => current_mat,
                },
                // 4. REALCE DE BORDES Y SEGMENTACIÓN
                4 => match option {
                    1 => EdgeManager::sharpen(&current_mat)?,
                    2 => EdgeManager::laplacian(&current_mat)?,
                    3 => EdgeManager::canny(&current_mat, 50.0, 150.0)?,
                    _ => current_mat,
                },
                // 5. TRANSFORMACIONES AFINES (Geometría)
                5 => match option {
                    1 => NixAffine::translate(&current_mat, 50.0, 50.0)?, // Traslación fija de prueba
                    2 => {
                        let engine = NixAffine::new(&current_mat, 1.0)?;
                        engine.rotate(&current_mat, 45.0)? // Rotación de 45°
                    },
                    _ => current_mat,
                },
                // 6. EXTRACCIÓN DE CONTORNOS (Paso final de análisis)
                6 => {
                    let mut display = current_mat.clone();
                    // Importante: find_and_measure dibuja sobre la imagen
                    let _metrics = NixContour::find_and_measure(&current_mat, &mut display)?;
                    display // Retornamos la imagen con los contornos dibujados
                },
                _ => current_mat,
            };
        }
        Ok(current_mat)
    }
}