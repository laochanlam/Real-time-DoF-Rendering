extern crate image;
extern crate num;
extern crate num_traits;

mod cayon;   // import test.rs

fn main() {
    let img = image::open("data/input.bmp").unwrap();

    // call library
    // TODO: figure out the value of sigma
    let sigma = 5f32;
    let blur_img = img.blur(sigma);
    let ref mut fout = std::fs::File::create("data/blurred.bmp").unwrap();
    blur_img.save(fout, image::BMP).unwrap();

    // call my function
    let red_img = cayon::redden(img);
    red_img.save("data/redden.bmp");

}
