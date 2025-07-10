mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;

// fn main() {
//     let width = 200;
//     let height = 200;
//     let background_color= Color::new(50, 50, 100, 255);
//     let mut framebuffer = Framebuffer::new(width, height, background_color);
 
//     framebuffer.set_background_color(Color::new(50, 50, 100, 255));
//     framebuffer.clear();
 
//     framebuffer.set_current_color(Color::GREEN);
//     line(
//         &mut framebuffer,
//         Vector2::new(50.0, 50.0),
//         Vector2::new(350.0, 350.0),
//     );
 
//     framebuffer.set_current_color(Color::RED);
//     line(
//         &mut framebuffer,
//         Vector2::new(150.0, 150.0),
//         Vector2::new(350.0, 350.0),
//     );
 
//     let output_file = "out.bmp";
 
//     framebuffer.render_to_file(output_file);
// }

fn main(){
    let width = 400;
    let height = 400;
    let background_color= Color::BLACK;
    let mut framebuffer = Framebuffer::new(width, height, background_color);
 
    framebuffer.set_background_color(background_color);
    framebuffer.clear();

    let array1 = [
        Vector2::new(165.0, 380.0),
        Vector2::new (185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];     

    draw_polygon(&mut framebuffer, Color::WHITE, &array1);

    let output_file = "out.bmp";
 
    framebuffer.render_to_file(output_file);
}