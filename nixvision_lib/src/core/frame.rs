
impl NixFrame {
    /// Manipulación directa de píxeles (movimiento de bits)
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut [u8]> {
        let channels = 3;
        if x < self.width as usize && y < self.height as usize {
            let index = (y * self.width as usize + x) * channels;
            Some(&mut self.data[index..index + channels])
        } else {
            None
        }
    }
}