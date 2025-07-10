use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2,){
    
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let steps = dx.abs().max(dy.abs()) as i32;

    let x_inc = dx / steps as f32;
    let y_inc = dy / steps as f32;

    let mut x = start.x;
    let mut y = start.y;

    for _ in 0..=steps {
        framebuffer.set_pixel(x.round() as i32, y.round() as i32);
        x += x_inc;
        y += y_inc;
    }

}