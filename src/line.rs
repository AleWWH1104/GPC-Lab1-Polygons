use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2,){
    // let xo = start.x as i32;
    // let xf = end.x as i32;
    // let yo = start.y as i32;
    // let yf = end.y as i32;

    // framebuffer.set_pixel(xo, yo);

    // let mut i = 0.0;

    // while i < 20.0 {
    //     let x = xo + i as i32;
    //     let y = yo + i as i32;

    //     framebuffer.set_pixel(x, y);

    //     i += 1.0;
    // }

    // framebuffer.set_pixel(xf, yf);
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