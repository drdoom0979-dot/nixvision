use opencv::{
    core::{Vector, Point, Scalar, Mat, Rect},
    imgproc,
    Result
};

pub struct ContourMetrics {
    pub area: f64,
    pub perimeter: f64,
    pub bbox: Rect,
    pub index: i32, // Añadimos el índice para poder resaltar después
}

pub struct NixContour;

impl NixContour {
    /// Detecta y mide contornos filtrando por un área mínima[cite: 5, 15, 26].
    pub fn find_and_measure(
        edges: &Mat, 
        original: &mut Mat, 
        min_area: f64 // Ahora recibe el parámetro del pipeline
    ) -> Result<Vec<ContourMetrics>> {
        let mut contours = Vector::<Vector<Point>>::new();
        let mut results = Vec::new();

        // 1. Encontrar contornos externos (Paso 4) [cite: 21]
        imgproc::find_contours(
            edges, 
            &mut contours, 
            imgproc::RETR_EXTERNAL, 
            imgproc::CHAIN_APPROX_SIMPLE, 
            Point::new(0, 0)
        )?;

        for i in 0..contours.len() {
            let cnt = contours.get(i)?;
            
            // 2. Calcular Área (Paso 5) 
            let area = imgproc::contour_area(&cnt, false)?;

            // 3. Filtrado dinámico para evitar ruido (Paso 6) [cite: 15, 26]
            if area > min_area { 
                // 4. Calcular Perímetro y Caja delimitadora (Paso 5) [cite: 16, 24, 25]
                let perimeter = imgproc::arc_length(&cnt, true)?; 
                let bbox = imgproc::bounding_rect(&cnt)?;       
                
                // Dibujar contornos detectados en Verde [cite: 21]
                imgproc::draw_contours(
                    original, &contours, i as i32, 
                    Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, &Mat::default(), 0, Point::new(0, 0)
                )?;

                results.push(ContourMetrics { area, perimeter, bbox, index: i as i32 });
            }
        }

        Ok(results)
    }

    /// Método para resaltar específicamente el contorno más grande (Paso 7).
    pub fn draw_highlight(original: &mut Mat, target: &ContourMetrics) -> Result<()> {
        // Dibujamos un rectángulo (Bounding Box) o resaltamos el contorno en Rojo [cite: 25, 27]
        imgproc::rectangle(
            original, 
            target.bbox, 
            Scalar::new(0.0, 0.0, 255.0, 0.0), // Rojo
            3, 
            imgproc::LINE_8, 
            0
        )?;
        
        Ok(())
    }
}