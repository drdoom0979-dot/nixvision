use opencv::{imgproc,
    prelude::*, 
    core, 
    Result
};

pub struct NoiseReducer;

impl NoiseReducer {
    /// FILTRO 1: Gaussian Blur
    /// Es el estándar para reducir ruido de alta frecuencia (grano).
    /// Tiende a difuminar los bordes del objeto patrón[cite: 52].
    pub fn gaussian(src: &Mat, kernel_size: i32) -> Result<Mat> {
        let mut dst = Mat::default(); // Prepara el contenedor para la imagen filtrada.
        imgproc::gaussian_blur_def(
            src, 
            &mut dst, 
            core::Size::new(kernel_size, kernel_size), 
            0.0 
        )?; // El '?' significa: "Si esto falla, devuelve el error inmediatamente".
        Ok(dst) // Si todo salió bien, devuelve la matriz envuelta en Ok.
    }

    /// FILTRO 2: Median Blur
    /// Muy efectivo para el ruido tipo "sal y pimienta" (puntos blancos/negros)[cite: 53].
    pub fn median(src: &Mat, kernel_size: i32) -> Result<Mat> {
        let mut dst = Mat::default();
        imgproc::median_blur(src, &mut dst, kernel_size)?;
        Ok(dst)
    }

    /// FILTRO 3: Bilateral Filter (Recomendado para el reporte)
    /// Es un filtro que reduce el ruido pero PRESERVA LOS BORDES.
    /// Ideal para que el Nothing Phone 2a no pierda nitidez en el objeto patrón.
    pub fn bilateral(src: &Mat, d: i32, sigma_color: f64, sigma_space: f64) -> Result<Mat> {
        let mut dst = Mat::default();
        imgproc::bilateral_filter(
            src, 
            &mut dst, 
            d, 
            sigma_color, 
            sigma_space, 
            core::BORDER_DEFAULT
        )?;
        Ok(dst)
    }
}