use opencv::{
    core, 
    imgproc, 
    prelude::*, 
    Result
};

pub struct QualityMetrics;

impl QualityMetrics {
    pub fn calculate(src: &Mat) -> Result<(f64, f64, f64)> {
        // 1. Contraste: Desviación estándar
        let mut mean = core::Scalar::default();
        let mut stddev = core::Scalar::default();
        core::mean_std_dev(src, &mut mean, &mut stddev, &core::no_array())?;

        // 2. Nitidez: Varianza del Laplaciano
        let mut laplacian_dst = Mat::default();
        imgproc::laplacian(src, &mut laplacian_dst, core::CV_64F, 3, 1.0, 0.0, core::BORDER_DEFAULT)?;
        let mut l_mean = core::Scalar::default();
        let mut l_stddev = core::Scalar::default();
        core::mean_std_dev(&laplacian_dst, &mut l_mean, &mut l_stddev, &core::no_array())?;
        
        let sharpness = l_stddev[0] * l_stddev[0]; // Varianza
        let snr = if stddev[0] > 0.0 { mean[0] / stddev[0] } else { 0.0 };

        Ok((stddev[0], sharpness, snr))
    }
}