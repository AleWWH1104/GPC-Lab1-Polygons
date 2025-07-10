mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;
use polygon::fill_polygon;

fn main(){
    let width = 400;
    let height = 400;
    let background_color= Color::BLACK;
    let mut framebuffer = Framebuffer::new(width, height, background_color);
 
    framebuffer.set_background_color(background_color);
    framebuffer.clear();

    let array2 = [
        Vector2::new(321.0, 335.0),
        Vector2::new (288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];     

    fill_polygon(&mut framebuffer, Color::BLUE, &array2);
    draw_polygon(&mut framebuffer, Color::WHITE, &array2);

    let output_file = "out.bmp";
 
    framebuffer.render_to_file(output_file);
}