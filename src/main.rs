mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;
use polygon::fill_polygon;

fn main(){
    let width = 800;
    let height = 300;
    let background_color= Color::BLACK;
    let mut framebuffer = Framebuffer::new(width, height, background_color);
 
    framebuffer.set_background_color(background_color);
    framebuffer.clear();

    let array4 = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0) ,
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];  

    let array5 =[
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];
         
    fill_polygon(&mut framebuffer, Color::GREEN, &array4);
    fill_polygon(&mut framebuffer, background_color, &array5);
    draw_polygon(&mut framebuffer, Color::WHITE, &array4);
    draw_polygon(&mut framebuffer, Color::WHITE, &array5);


    let output_file = "out.bmp";
 
    framebuffer.render_to_file(output_file);
}