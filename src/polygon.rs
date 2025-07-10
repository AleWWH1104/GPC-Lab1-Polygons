use raylib::prelude::*;
use crate::framebuffer::*;
use crate::line::line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, color: Color, array: &[Vector2]){

    framebuffer.set_current_color(color);
    if array.len() < 2 {
        return; 
    }

    for i in 0..array.len() - 1 {
        line(framebuffer, array[i], array[i + 1]);
    }

    // Conectar el último punto con el primero para cerrar el polígono
    line(framebuffer, array[array.len() - 1], array[0]);
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, color: Color, array: &[Vector2]) {
    framebuffer.set_current_color(color);

    if array.len() < 3 {
        return; // No es un polígono
    }

    // Buscar el rango vertical del polígono
    let min_y = array.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = array.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    // Por cada fila de píxeles (scanline)
    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..array.len() {
            let p1 = array[i];
            let p2 = array[(i + 1) % array.len()];

            // Verifica si el borde cruza la línea actual (y)
            if (p1.y <= y as f32 && p2.y > y as f32) || (p2.y <= y as f32 && p1.y > y as f32) {
                let dy = p2.y - p1.y;
                if dy != 0.0 {
                    let t = (y as f32 - p1.y) / dy;
                    let x = p1.x + t * (p2.x - p1.x);
                    intersections.push(x.round() as i32);
                }
            }
        }

        // Ordenar las intersecciones de menor a mayor
        intersections.sort_unstable();

        // Rellenar entre pares de intersecciones
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0];
                let x_end = pair[1];
                for x in x_start..=x_end {
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }
}