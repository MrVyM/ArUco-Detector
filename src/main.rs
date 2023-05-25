use image;
use image::{Pixel, GenericImageView, GenericImage};

const THREESOLD:u8= 10;
fn grayscale(img)
{
    for x in 0..img.width()
    {
        for y in 0..img.height()
        {
            let mut pixels = img.get_pixel(x,y);
            pixels[0] = 255;
            img.put_pixel(x,y, pixels);
        }
    }
}

#[warn(non_snake_case)]
fn main() {
    let mut img = image::open("assets/aruco_1.jpg").expect("File not found!");
    grayscale(img);
    img.save("output.jpg").unwrap();
}
