extern crate image;
extern crate num;
extern crate num_traits;
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;
extern crate time;

mod cayon;   // import test.rs

fn main() {
    let img = image::open("data/ds1.png").unwrap();
    let img_gray = image::open("data/ds2.png").unwrap();

    let mut start_time = time::get_time().sec;

    let mipmap_level: [f32; 5] = [1.46, 3.05, 6.2, 13.05, 24.83];
    let mut pathblur: [String; 5] = ["data/dst1.bmp".to_string(), "data/dst2.bmp".to_string(), "data/dst3.bmp".to_string(), "data/dst4.bmp".to_string(), "data/dst5.bmp".to_string()];
    let mut pathdown: [String; 5] = ["data/dst_downsize1.bmp".to_string(), "data/dst_downsize2.bmp".to_string(), "data/dst_downsize3.bmp".to_string(), "data/dst_downsize4.bmp".to_string(), "data/dst_downsize5.bmp".to_string()];

    let mut b = cayon::count_coc(&img_gray);
    let c = cayon::whatever(&img, &mut b);
    c.save("data/dst_haha.bmp");

    let mut end_time = time::get_time().sec;
    println!("time = {:?}", end_time - start_time);
}
