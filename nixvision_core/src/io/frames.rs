use opencv::{
    imgcodecs, 
    prelude::*, 
    Result,
    videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait},
    core::Mat,
};

use std::{thread, time::Duration};

pub struct FrameCapture;

impl FrameCapture{

    pub fn load_image(path: &str) -> Result<Mat> {
        imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)
    }

    pub fn save_image(img: &Mat, filename: &str) -> Result<bool> {
        imgcodecs::imwrite(filename, img, &opencv::core::Vector::new())
    }

    pub fn capture_from_stream(url: &str) -> opencv::Result<Mat> {
        use opencv::videoio::{VideoCapture, CAP_ANY, VideoCaptureTrait};
        
        // Abrir el stream
        let mut cam = VideoCapture::from_file(url, CAP_ANY)?;
        let mut frame = Mat::default();

        if cam.is_opened()? {
            cam.read(&mut frame)?; // Captura el frame actual
        }
        
        Ok(frame)
    }

    pub fn capture_sequence(url: &str, fps: f64, seconds: u64) -> opencv::Result<Vec<Mat>> {
        let mut cam = VideoCapture::from_file(url, CAP_ANY)?;
        let mut frames = Vec::new();
        
        // Calculamos cuántos frames totales necesitamos
        let total_frames = (fps * seconds as f64) as usize;
        // Intervalo de tiempo entre capturas
        let interval = Duration::from_millis((1000.0 / fps) as u64);

        if cam.is_opened()? {
            for _ in 0..total_frames {
                let mut frame = Mat::default();
                if cam.read(&mut frame)? {
                    frames.push(frame);
                }
                // Esperamos para cumplir con los FPS solicitados
                thread::sleep(interval);
            }
        }

        Ok(frames)
    }

}

