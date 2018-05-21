extern crate image;
extern crate num;
extern crate num_traits;
extern crate time;
mod cayon;   // import test.rs

fn main() {
    let start = time::get_time().sec;

    let input_img = image::open("data/input.bmp").unwrap();
    let gray_img = image::open("data/gray.bmp").unwrap();
    let mut coc: Vec<i32> = cayon::count_coc(&gray_img);
    let img_render = cayon::whatever(&input_img, &mut coc);
    img_render.save("data/haha.bmp");

    let end = time::get_time().sec;
    println!("{}", end-start);
}
