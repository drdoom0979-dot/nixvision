pub struct AssignmentConfig {
    pub translation_x: f32,
    pub translation_y: f32,
    pub rotation_angle: f64, // En grados para OpenCV [cite: 92]
}

pub struct CameraConfig {
    pub rtsp_url: &'static str,
    pub frame_width: i32,
    pub frame_height: i32,
}

pub const OFFICE_CAMERA: CameraConfig = CameraConfig {
    rtsp_url: "rtsp://admin:password@192.168.1.10:554/stream",
    frame_width: 1280,
    frame_height: 720,
};

pub struct RuntimeConfig {
    pub input_path: String,
    pub output_path: String,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
        }
    }
}