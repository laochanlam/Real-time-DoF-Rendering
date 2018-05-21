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
    let img_gray = image::open("data/1305031917.078672.png").unwrap();

    	//let img_gray = image::open("data/ds2.png").unwrap();
	/*let (width_g, height_g) = img_gray.dimensions();
	let (width_g, height_g) = (width_g as i32, height_g as i32);
	let mut distance_gray: Vec<i32> = vec![0; NumCast::from(width_g * height_g).unwrap()];
    let mut ey=0;
	for x in 0..width_g {
        for y in 0..height_g {
            let px = img_gray.get_pixel(x as u32, y as u32);
            let (k1, _, _, _) = px.channels4();
			let k1_unwrap: i32 = NumCast::from(k1).unwrap();
            if (k1>ey) {ey=k1;}
           
			distance_gray[(x*height_g+y) as usize] = k1_unwrap/60 + 1 as i32;
			//1-5
		} println!("g={:} ",ey);
    }*/




    let mut start_time = time::get_time().sec;

    println!("start = {:?}", time::get_time().sec);
    let mipmap_level: [f32; 5] = [1.46, 3.05, 6.2, 13.05, 24.83];
    let mut pathblur: [String; 5] = ["data/dst1.bmp".to_string(), "data/dst2.bmp".to_string(), "data/dst3.bmp".to_string(), "data/dst4.bmp".to_string(), "data/dst5.bmp".to_string()];
    let mut pathdown: [String; 5] = ["data/dst_downsize1.bmp".to_string(), "data/dst_downsize2.bmp".to_string(), "data/dst_downsize3.bmp".to_string(), "data/dst_downsize4.bmp".to_string(), "data/dst_downsize5.bmp".to_string()];

    /*for n in 0..5{
        let sigma = mipmap_level[n];
        println!("{:} {:}",n,sigma);
        let blur_img = img.blur(sigma);
        let ref mut fout = std::fs::File::create(pathblur[n].to_string()).unwrap();
        blur_img.save(fout, image::BMP).unwrap();

	    //call cayon's function
        let two: i32 = 2;
	    let downsize_img = cayon::downsize(blur_img, (n+1) as u32);
        downsize_img.save(pathdown[n].to_string());
    }*/

    let mut b = cayon::count_coc(&img_gray);
    let c = cayon::whatever(&img, &mut b);
    c.save("data/dst_haha.bmp");

    // call library
    // TODO: figure out the value of sigma
    let mut end_time = time::get_time().sec;
    println!("end = {:?}", end_time);
    println!("timetime = {:?}", end_time - start_time);

}
