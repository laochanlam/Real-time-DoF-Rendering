extern crate image;
extern crate num;
extern crate num_traits;

mod cayon;   // import test.rs

fn main() {

    let mipmap_level: [f32; 5] = [1.46, 3.05, 6.2, 13.05, 24.83];
    let mut pathblur: [String; 5] = ["data/blurred1.bmp".to_string(), "data/blurred2.bmp".to_string(), "data/blurred3.bmp".to_string(), "data/blurred4.bmp".to_string(), "data/blurred5.bmp".to_string()];
    let mut pathdown: [String; 5] = ["data/downsize1.bmp".to_string(), "data/downsize2.bmp".to_string(), "data/downsize3.bmp".to_string(), "data/downsize4.bmp".to_string(), "data/downsize5.bmp".to_string()];


    let mut img = image::open("data/input.bmp").unwrap();

    for n in 1..5{
        let sigma = mipmap_level[n];
        let blur_img = img.blur(sigma);
        //println!("{}",pathblur[n]);
        let ref mut fout = std::fs::File::create(pathblur[n-1].to_string()).unwrap();
        blur_img.save(fout, image::BMP).unwrap();

	    //call cayon's function
	    let downsize_img = cayon::downsize(blur_img, (n) as u32);
        downsize_img.save(pathdown[n-1].to_string());
    }
    // call library
    // TODO: figure out the value of sigma

}
