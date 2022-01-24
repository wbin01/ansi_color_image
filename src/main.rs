use ansi_term::Colour::RGB;
use ansi_color_image as aci;

fn main() {
    let url = "/usr/share/pixmaps/neon.png";
    let mut img = aci::ImageColorMap::new(url, Some(20), Some(40), Some(20.0), Some(-15));
    //                                   image height    width     contrast    brightness

    for pixel_line  in img.build_pixel_map() { // pixel_line  =  [pixel, pixel, pixel]
        for pixel in pixel_line {             //  pixel       =  ("*", (255, 255, 255))   
            let (char_, rgb) = pixel;
            print!("{}", RGB(rgb.0, rgb.1, rgb.2).paint(char_));  // Print without newline 
        }
        println!();  // New line
    }
}
