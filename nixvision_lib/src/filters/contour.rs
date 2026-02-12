use opencv::{
    core::{Vector, Point, Scalar, Mat, Rect},
    imgproc,
    Result
};

pub struct ContourMetrics {
    pub area: f64,
    pub perimeter: f64,
    pub bbox: Rect,
}

pub struct NixContour;

impl NixContour {
    /// Detecta, mide y resalta contornos en tiempo real[cite: 5, 21].
    pub fn find_and_measure(edges: &Mat, original: &mut Mat) -> Result<Vec<ContourMetrics>> {
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

        let mut max_area = 0.0;
        let mut max_idx: i32 = -1;

        for i in 0..contours.len() {
            let cnt = contours.get(i)?;
            
            // 2. Calcular Área (Paso 5) [cite: 23]
            let area = imgproc::contour_area(&cnt, false)?;

            // 3. Filtrar por área mínima para evitar ruido (Paso 6) 
            if area > 1000.0 { 
                // 4. Calcular Perímetro y Caja delimitadora (Paso 5) [cite: 24, 25]
                let perimeter = imgproc::arc_length(&cnt, true)?; 
                let bbox = imgproc::bounding_rect(&cnt)?;       
                
                // 5. Lógica para detectar el contorno más grande (Paso 7) [cite: 27]
                if area > max_area {
                    max_area = area;
                    max_idx = i as i32;
                }

                // Dibujar contornos detectados en Verde [cite: 21]
                imgproc::draw_contours(
                    original, &contours, i as i32, 
                    Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, &Mat::default(), 0, Point::new(0, 0)
                )?;

                results.push(ContourMetrics { area, perimeter, bbox });
            }
        }

        // 6. Resaltar el contorno más grande en Rojo (Extensión Paso 7) [cite: 27]
        if max_idx != -1 {
            imgproc::draw_contours(
                original, &contours, max_idx, 
                Scalar::new(0.0, 0.0, 255.0, 0.0), 4, 8, &Mat::default(), 0, Point::new(0, 0)
            )?;
        }

        Ok(results)
    }
}

