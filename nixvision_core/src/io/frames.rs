use opencv::{imgcodecs, prelude::*, Result};

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

}

