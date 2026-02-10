use opencv::prelude::*;

pub struct NixFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl NixFrame {
    /// Convierte una Mat de OpenCV a un NixFrame para manipulación de bits.
    /// Esto permite trabajar con imágenes de archivos [cite: 122] o streams RTSP.
    pub fn mat_to_nix(matrix: &Mat) -> opencv::Result<Self> {
        let size = matrix.size()?;
        // Reservamos espacio: Ancho * Alto * 3 canales (BGR) [cite: 202]
        let mut data = Vec::with_capacity((size.width * size.height * 3) as usize);
        
        // Acceso directo a los bytes crudos (movimiento de bits) hacia la ram
        let data_bytes = matrix.data_bytes()?;
        data.extend_from_slice(data_bytes);

        Ok(Self {
            width: size.width as u32,
            height: size.height as u32,
            data,
        })
    }
}