use opencv::{core, imgproc, prelude::*};

pub struct NixAffine {
    // Podemos guardar el tamaño de la imagen o parámetros por defecto
    pub center: core::Point2f,
    pub scale: f64,
}

impl NixAffine {
    /// Constructor para inicializar la transformación con una imagen específica
    pub fn new(src: &Mat, scale: f64) -> opencv::Result<Self> {
        let size = src.size()?;
        Ok(Self {
            center: core::Point2f::new(size.width as f32 / 2.0, size.height as f32 / 2.0),
            scale,
        })
    }

    /// Implementa la traslación de la imagen
    pub fn translate(src: &Mat, tx: f32, ty: f32) -> opencv::Result<Mat> {
        let mut dst = Mat::default();
        // Matriz de traslación 2x3
        let transformation_matrix = core::Mat::from_slice_2d(&[
            &[1.0, 0.0, tx],
            &[0.0, 1.0, ty]
        ])?;
        
        imgproc::warp_affine(
            src, 
            &mut dst, 
            &transformation_matrix, 
            src.size()?, 
            imgproc::INTER_LINEAR, 
            core::BORDER_CONSTANT, 
            core::Scalar::default()
        )?;
        Ok(dst)
    }

    /// Implementa la rotación usando el estado del struct
    pub fn rotate(&self, src: &Mat, angle_deg: f64) -> opencv::Result<Mat> {
        let mut dst = Mat::default();
        
        // NOTA: Recuerda el guion bajo: get_rotation_matrix_2d
        let rotation_matrix = imgproc::get_rotation_matrix_2d(
            self.center, 
            angle_deg, 
            self.scale
        )?;
        
        imgproc::warp_affine(
            src, 
            &mut dst, 
            &rotation_matrix, 
            src.size()?, 
            imgproc::INTER_LINEAR, 
            core::BORDER_CONSTANT, 
            core::Scalar::default()
        )?;
        Ok(dst)
    }
}