use opencv::{imgproc, prelude::*, Result};

pub struct ColorConverter;

impl ColorConverter {
    /// Convierte una imagen de BGR (estándar de OpenCV) a Escala de Grises.
    /// Esto cumple con el Paso 1 de la Actividad B.
    pub fn to_grayscale(src: &Mat) -> Result<Mat> {
        let mut dst = Mat::default();
        imgproc::cvt_color_def(src, &mut dst, imgproc::COLOR_BGR2GRAY)?;
        Ok(dst)
    }

    /// Convierte la imagen a espacio de color HSV.
    /// Útil para el análisis de iluminación mediante el canal V (Value).
    pub fn to_hsv(src: &Mat) -> Result<Mat> {
        let mut dst = Mat::default();
        imgproc::cvt_color_def(src, &mut dst, imgproc::COLOR_BGR2HSV)?;
        Ok(dst)
    }

    /// Extrae únicamente el canal V (Value/Brillo) de una imagen HSV.
    /// Esto te servirá para la justificación técnica en el reporte sobre la iluminación.
    pub fn extract_v_channel(hsv_img: &Mat) -> Result<Mat> {
        let mut channels = opencv::core::Vector::<Mat>::new();
        opencv::core::split(hsv_img, &mut channels)?;
        // El canal V es el índice 2 en HSV (H=0, S=1, V=2)
        Ok(channels.get(2)?)
    }
}