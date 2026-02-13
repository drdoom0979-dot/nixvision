use opencv::{
    core::{Vector, Point, Scalar, Mat, Rect},
    imgproc,
    Result
};

pub struct ContourMetrics {
    pub area: f64,
    pub perimeter: f64,
    pub bbox: Rect,
    pub width: i32,  // Añadido para Punto 5 [cite: 25]
    pub height: i32, // Añadido para Punto 5 [cite: 25]
    pub index: i32,
}

pub struct NixContour;

impl NixContour {
    /// Detecta y mide contornos filtrando por un área mínima[cite: 5, 15, 26].
    pub fn find_and_measure(
        edges: &Mat, 
        original: &mut Mat, 
        min_area: f64 
    ) -> Result<Vec<ContourMetrics>> {
        let mut contours = Vector::<Vector<Point>>::new();
        let mut results = Vec::new();

        // 1. Encontrar contornos externos [cite: 14, 21]
        imgproc::find_contours(
            edges, 
            &mut contours, 
            imgproc::RETR_EXTERNAL, 
            imgproc::CHAIN_APPROX_SIMPLE, 
            Point::new(0, 0)
        )?;

        for i in 0..contours.len() {
            let cnt = contours.get(i)?;
            let area = imgproc::contour_area(&cnt, false)?;

            // 2. Filtrado para evitar ruido (Paso 6) [cite: 15, 26]
            if area > min_area { 
                let perimeter = imgproc::arc_length(&cnt, true)?; 
                let bbox = imgproc::bounding_rect(&cnt)?; // Cálculo de Bounding Box [cite: 25]
                
                // 3. Visualización de contornos detectados en Verde [cite: 21]
                imgproc::draw_contours(
                    original, &contours, i as i32, 
                    Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, &Mat::default(), 0, Point::new(0, 0)
                )?;

                // 4. Almacenar métricas completas (Punto 5) 
                results.push(ContourMetrics { 
                    area, 
                    perimeter, 
                    bbox, 
                    width: bbox.width,   // Extraemos el ancho
                    height: bbox.height, // Extraemos el alto
                    index: i as i32 
                });
            }
        }
        Ok(results)
    }

    /// Resalta el contorno más grande con un rectángulo rojo (Paso 7).
    pub fn draw_highlight(original: &mut Mat, target: &ContourMetrics) -> Result<()> {
        imgproc::rectangle(
            original, 
            target.bbox, 
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Rojo 
            5, // Grosor aumentado para visibilidad
            imgproc::LINE_8, 
            0
        )?;
        Ok(())
    }
}