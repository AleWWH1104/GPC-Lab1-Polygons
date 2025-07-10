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