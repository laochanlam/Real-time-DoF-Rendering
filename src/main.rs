extern crate image;

fn main() {
    let img = image::open("data/input.bmp").unwrap();
    let sigma = 5f32;
    //TODO: figure out the value of sigma
    let img_out = img.blur(sigma);
    let ref mut fout = std::fs::File::create("data/output.bmp").unwrap();
    img_out.save(fout, image::BMP).unwrap();

}
