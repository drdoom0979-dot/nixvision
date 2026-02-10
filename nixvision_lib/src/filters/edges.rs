use opencv::{imgproc, prelude::*, core, Result}; // Importamos herramientas de procesamiento, núcleos y manejo de errores

pub struct EdgeManager; // Estructura para agrupar las utilidades de realce y bordes

impl EdgeManager {
    /// Aplica un filtro de Sharpen (Afilado) usando una máscara de convolución.
    /// Esto ayuda a resaltar los bordes del objeto patrón (regla, PCB, etc.)[cite: 21, 56].
    pub fn sharpen(src: &Mat) -> Result<Mat> {
        let mut dst = Mat::default(); // Matriz de destino vacía
        
        // Definimos el kernel de afilado (Laplaciano negativo central)
        // [ 0, -1,  0]
        // [-1,  5, -1]
        // [ 0, -1,  0]
        let kernel = Mat::from_slice_2d(&[
            &[0.0f32, -1.0, 0.0],
            &[-1.0, 5.0, -1.0],
            &[0.0, -1.0, 0.0],
        ])?;

        // filter_2d aplica la máscara (kernel) sobre la imagen
        imgproc::filter_2d(
            src,                // Imagen de entrada (usualmente ya filtrada contra ruido)
            &mut dst,           // Destino
            -1,                 // Profundidad de salida (-1 mantiene la misma que la entrada)
            &kernel,            // Nuestra máscara de afilado
            core::Point::new(-1, -1), // Ancla en el centro del kernel
            0.0,                // Valor delta sumado al resultado
            core::BORDER_DEFAULT, // Manejo de bordes de la imagen
        )?;
        
        Ok(dst) // Retorna la imagen con bordes más definidos envuelta en Ok
    }

    /// Calcula el Laplaciano suave para detectar bordes o medir nitidez[cite: 56, 63].
    pub fn laplacian(src: &Mat) -> Result<Mat> {
        let mut dst = Mat::default(); // Matriz de destino
        
        // El Laplaciano resalta zonas de cambio rápido de intensidad (bordes)
        imgproc::laplacian(
            src,                // Imagen de entrada
            &mut dst,           // Imagen de salida
            core::CV_16S,       // Usamos 16 bits con signo para evitar pérdida de datos negativos
            3,                  // Tamaño del kernel (3x3)
            1.0,                // Escala
            0.0,                // Delta
            core::BORDER_DEFAULT,
        )?;

        // Convertimos de nuevo a 8 bits (absolutos) para poder visualizarla
        let mut abs_dst = Mat::default();
        core::convert_scale_abs(&dst, &mut abs_dst, 1.0, 0.0)?;
        
        Ok(abs_dst) // Retorna la imagen de bordes
    }
}