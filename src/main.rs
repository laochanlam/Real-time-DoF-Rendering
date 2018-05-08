extern crate image;
extern crate num;
extern crate num_traits;
mod cayon;   // import test.rs

fn main() {
    let input_img = image::open("data/input.bmp").unwrap();
    let gray_img = image::open("data/gray.bmp").unwrap();
    let mut coc: Vec<i32> = cayon::count_coc(&gray_img);
    let img_render = cayon::whatever(&input_img, &mut coc);
    img_render.save("data/haha.bmp");
}
