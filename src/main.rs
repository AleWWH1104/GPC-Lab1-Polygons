mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;
use polygon::fill_polygon;
fn main(){
    let width = 500;
    let height = 300;
    let background_color= Color::BLACK;
    let mut framebuffer = Framebuffer::new(width, height, background_color);
 
    framebuffer.set_background_color(background_color);
    framebuffer.clear();

    let array3 = [
        Vector2::new(377.0, 249.0) ,
        Vector2::new (411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];     

    fill_polygon(&mut framebuffer, Color::RED, &array3);
    draw_polygon(&mut framebuffer, Color::WHITE, &array3);

    let output_file = "out.bmp";
 
    framebuffer.render_to_file(output_file);
}