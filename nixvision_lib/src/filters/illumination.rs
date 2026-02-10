use opencv::{imgproc, prelude::*, core, Result};

pub struct IlluminationManager;

impl IlluminationManager {
    /// OPCIÓN 1: Normalización por canal (Min-Max) 
    /// Escala los valores de intensidad para que cubran el rango completo [0, 255].
    pub fn normalize(src: &Mat) -> Result<Mat> {
    let mut dst = Mat::default(); // Crea la matriz de destino vacía.
    core::normalize(
        src,                    // Imagen de entrada (grises).
        &mut dst,               // Donde se guarda el resultado.
        0.0,                    // Valor mínimo deseado (negro).
        255.0,                  // Valor máximo deseado (blanco).
        core::NORM_MINMAX,      // Algoritmo: estiramiento lineal entre el min y el max.
        core::CV_8U,            // Tipo de dato: 8 bits sin signo (0-255).
        &core::no_array(),     // Máscara opcional (no usamos ninguna).
    )?;
    Ok(dst) // Retorna la imagen estirada.
}

    /// OPCIÓN 2: Corrección por "fondo" (Flat-field aproximado) 
    /// Estima el fondo usando un blur grande y divide la imagen original entre este fondo.
    /// Muy útil para eliminar viñeteado o iluminación desigual del sensor.
    pub fn background_correction(src: &Mat) -> Result<Mat> {
        
        let mut src_f32 = Mat::default();
        let mut background_f32 = Mat::default();
        let mut dst_f32 = Mat::default();
        let mut dst = Mat::default();

        // Convertimos a 32 bits flotantes (f32) porque al dividir píxeles necesitamos decimales.
        src.convert_to(&mut src_f32, core::CV_32F, 1.0, 0.0)?;

        // Creamos un "mapa de iluminación" aplicando un desenfoque muy fuerte. 
        // Esto borra el objeto patrón y solo deja la mancha de luz del fondo.
        imgproc::gaussian_blur_def(
            &src_f32, 
            &mut background_f32, 
            core::Size::new(101, 101), 
            0.0
        )?;

        // Calculamos el brillo promedio de la imagen original.
        let mean_val = core::mean(&src_f32, &core::no_array())?;

        // Dividimos la imagen original entre el fondo. 
        // Si un píxel es oscuro por una sombra del fondo, al dividir se vuelve más claro.
        opencv::core::divide(mean_val[0], &background_f32, &mut dst_f32, -1)?;

        // Volvemos a convertir a 8 bits (0-255) para poder visualizarla.
        dst_f32.convert_to(&mut dst, core::CV_8U, 1.0, 0.0)?;
        Ok(dst)
    }

    /// OPCIÓN 3: CLAHE (Contrast Limited Adaptive Histogram Equalization) 
    /// Divide la imagen en rejillas y ecualiza localmente. 
    /// Es la opción más robusta para el Escenario 3 (Iluminación no uniforme).
    pub fn apply_clahe(src: &Mat, clip_limit: f64, grid_size: core::Size) -> Result<Mat> {
        let mut dst = Mat::default();
        
        // create_clahe crea el motor del algoritmo.
        // clip_limit: controla qué tanto contraste añadir (si es muy alto, sale ruido).
        // grid_size: en cuántos cuadritos divide la imagen para analizarla localmente.
        let mut clahe = imgproc::create_clahe(clip_limit, grid_size)?;
        
        // Aplica el algoritmo sobre la imagen de entrada.
        clahe.apply(src, &mut dst)?;
        
        Ok(dst)
    }
}