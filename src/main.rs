#![allow(unused_imports)]

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
    let (width, height) = img.dimensions();
    let (width, height) = (width as i32, height as i32);

    let start_time = time::get_time().sec;

    // let mipmap_level: [f32; 5] = [1.46, 3.05, 6.2, 13.05, 24.83];
    // let mut pathblur: [String; 5] = ["data/dst1.bmp".to_string(), "data/dst2.bmp".to_string(), "data/dst3.bmp".to_string(), "data/dst4.bmp".to_string(), "data/dst5.bmp".to_string()];
    // let mut pathdown: [String; 5] = ["data/dst_downsize1.bmp".to_string(), "data/dst_downsize2.bmp".to_string(), "data/dst_downsize3.bmp".to_string(), "data/dst_downsize4.bmp".to_string(), "data/dst_downsize5.bmp".to_string()];
    let _ = cayon::downsize(&img, 0);

    let mut dof = cayon::get_dof(&img_gray);
    let mut coc = cayon::get_coc(&mut dof, 0, 0, width, height);
    let new_img = cayon::render(&img, &mut coc);
    let _result = new_img.save("data/dst_haha.bmp");

    let end_time = time::get_time().sec;
    println!("time = {:?}", end_time - start_time);
}
