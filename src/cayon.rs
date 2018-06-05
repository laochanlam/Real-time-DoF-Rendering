extern crate image;
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;
use std::thread;
use std::sync::{Arc, Mutex};
static NTHREADS: i32 = 4;

pub fn count_coc <I: GenericImage> (img: &I) -> Vec<i32> 
    where I::Pixel: 'static {
    
    let (width, height) = img.dimensions();
	let _size = NumCast::from(width * height).unwrap();
    let mut coc: Vec<i32> = vec![0; _size];

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let (k1, _, _, _) = px.channels4();
            let k1_unwrap: f64 = NumCast::from(k1).unwrap();
            let mut radius = (k1_unwrap + 1.0).sqrt() as i32;
            if radius % 2 == 0 {
                radius += 1;
            }
			coc[(x*height + y) as usize] = radius;
            // assert_eq!(a[a.len()-1], 0);
        }
    }

    // return
    coc
}

pub fn copy_from_gi<O: GenericImage>(other: O)
	-> ImageBuffer<O::Pixel, Vec<<O::Pixel as Pixel>::Subpixel>>   
    where O::Pixel: 'static {

	let mut buf = ImageBuffer::new(other.width(), other.height());
	for i in 0 .. other.width() {
		for k in 0 .. other.height() {
			unsafe {
				let p = other.get_pixel(i, k);
				buf.put_pixel(i, k, p);
			}
		}
	}
    
	// return
	buf
}
pub fn copy_from_di<O: GenericImage>(other: O)
	-> ImageBuffer<O::Pixel, Vec<<O::Pixel as Pixel>::Subpixel>>   
    where O::Pixel: 'static {

	let mut buf = ImageBuffer::new(other.width(), other.height());
	for i in 0 .. other.width() {
		for k in 0 .. other.height() {
			unsafe {
				let p = other.get_pixel(i, k);
				buf.put_pixel(i, k, p);
			}
		}
	}

	// return 
    buf
}


pub fn whatever <I: GenericImage> (img: &I, radius: &mut Vec<i32>)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static {

	let (width, height) = img.dimensions();
	let (width, height) = (width as i32, height as i32);
	let _size = (width * height) as usize;
	let mut img_out = ImageBuffer::new(width as u32, height as u32);
	let mut pixel_r = vec![0.0; _size];
	let mut pixel_g = vec![0.0; _size];
	let mut pixel_b = vec![0.0; _size];
	let pixel_r = Arc::new(Mutex::new(pixel_r));
	let pixel_g = Arc::new(Mutex::new(pixel_g));
	let pixel_b = Arc::new(Mutex::new(pixel_b));

	let mut children = vec![];

	for id in 0..NTHREADS {
		let pixel_r = pixel_r.clone();
		let pixel_g = pixel_g.clone();
		let pixel_b = pixel_b.clone();
		let x_start = width / NTHREADS * id;
		let x_end   = width / NTHREADS * (id+1);
		let radius = radius.clone();

		let child = thread::spawn(move || {
			let mut local_pixel_r = vec![0.0; _size];
			let mut local_pixel_g = vec![0.0; _size];
			let mut local_pixel_b = vec![0.0; _size];

			let mut pathdown: [String; 5] = ["data/dst_downsize1.bmp".to_string(), "data/dst_downsize2.bmp".to_string(), "data/dst_downsize3.bmp".to_string(), "data/dst_downsize4.bmp".to_string(), "data/dst_downsize5.bmp".to_string()];
			let mut img = image::open("data/ds1.png").unwrap();
			let mut img_d1= image::open(pathdown[0].to_string()).unwrap();
			let mut img_d2= image::open(pathdown[1].to_string()).unwrap();
			let mut img_d3= image::open(pathdown[2].to_string()).unwrap();	
			let mut img_d4= image::open(pathdown[3].to_string()).unwrap();
			let mut img_d5= image::open(pathdown[4].to_string()).unwrap();

			let img_gray = image::open("data/ds2.png").unwrap();
			let (width_g, height_g) = img_gray.dimensions();
			let (width_g, height_g) = (width_g as i32, height_g as i32);
			let mut distance_gray: Vec<i32> = vec![0; NumCast::from(width_g * height_g).unwrap()];
			for x in 0..width_g {
				for y in 0..height_g {
					let px = img_gray.get_pixel(x as u32, y as u32);
					let (k1, _, _, _) = px.channels4();
					let k1_unwrap: i32 = NumCast::from(k1).unwrap();
					distance_gray[(x*height_g+y) as usize] = k1_unwrap/60 + 1 as i32;
					//1-5
				}
			}

			for x in x_start..x_end {
				for y in 0..height {
					let px = img.get_pixel(x as u32, y as u32);
					let (k1, k2, k3, k4) = px.channels4();
					let tup: (f64, f64, f64, f64) = (
						NumCast::from(k1).unwrap(),
						NumCast::from(k2).unwrap(),
						NumCast::from(k3).unwrap(),
						NumCast::from(k4).unwrap()
					);

					let d_lvl = distance_gray[(x*height_g+y) as usize];
					let mut selected_img;

					if d_lvl == 1{
						selected_img = &img;
					}else if d_lvl  == 2{
						selected_img = &img_d1;
					}else if d_lvl  == 3{
						selected_img = &img_d2;
					}else if d_lvl  == 4{
						selected_img = &img_d3;
					}else {
						selected_img = &img_d4;
					}

					let two: u32 =2; 
					let Q = (two.pow((d_lvl-1) as u32) as u32) as i32;			
					let px = selected_img.get_pixel((x/Q) as u32, (y/ Q)as u32);
					let (k1, k2, k3, k4) = px.channels4();
					let tup: (f64, f64, f64, f64) = (
						NumCast::from(k1).unwrap(),
						NumCast::from(k2).unwrap(),
						NumCast::from(k3).unwrap(),
						NumCast::from(k4).unwrap()
					);
			
					// (0, 0, 0) is black
					let _pos = (x * height + y) as usize;
					let r: i32 = radius[_pos as usize] as i32;
					let x_left:  i32 = x - (r-1)/2;
					let x_right: i32 = x + (r-1)/2;
					let y_left:  i32 = y - (r-1)/2;
					let y_right: i32 = y + (r-1)/2;

					for i in x_left..x_right {
						for j in y_left..y_right {
							if i<0 || i>= width || j<0 || j>= height {
								continue;
							}
							let r: i32 = radius[_pos];
							let x_left:  i32 = x - (r-1)/2;
							let x_right: i32 = x + (r-1)/2;
							let y_left:  i32 = y - (r-1)/2;
							let y_right: i32 = y + (r-1)/2;

							for i in x_left..x_right {
								for j in y_left..y_right {
									if i<0 || i>= width || j<0 || j>= height || i*height + j >= 1920000 {
										continue;
									}

									local_pixel_r[(i*height + j) as usize] += tup.0 / ((r*r) as f64);
									local_pixel_g[(i*height + j) as usize] += tup.1 / ((r*r) as f64);
									local_pixel_b[(i*height + j) as usize] += tup.2 / ((r*r) as f64);
								}
							}
						}
						// println!("id {}: end of a row", id);
					}
				}
				let mut pixel_r = pixel_r.lock().unwrap();
				let mut pixel_g = pixel_g.lock().unwrap();
				let mut pixel_b = pixel_b.lock().unwrap();
				for i in 0..width {
					for j in 0..height {
						let _pos = (i*height+j) as usize;
						pixel_r[_pos] += local_pixel_r[_pos];
						pixel_g[_pos] += local_pixel_g[_pos];
						pixel_b[_pos] += local_pixel_b[_pos];
					}
				}
				println!("hello: {}", x);
			}	// end of a thread

		});	// end of child = spawn a thread

		children.push(child);
	}

	for child in children {
		let _ = child.join();
	}

	let mut pixel_r = pixel_r.lock().unwrap();
	let mut pixel_g = pixel_g.lock().unwrap();
	let mut pixel_b = pixel_b.lock().unwrap();

	for x in 0..width {
		for y in 0..height {
			let _pos = (x*height + y) as usize;
			let r = pixel_r[_pos];
			let g = pixel_g[_pos];
			let b = pixel_b[_pos];
			let a = 0.0;

			let new_pixel = Pixel::from_channels(
                NumCast::from(clamp(r, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(g, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(b, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(a, 0.0, 255.0)).unwrap()
            );

            img_out.put_pixel(x as u32, y as u32, new_pixel);
		}
	}

	// return 
	img_out
}

pub fn downsize <I: GenericImage> (img: I, lvl: u32) 
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>   
    where I::Pixel: 'static {                                      
	let two: u32 =2;
    let (width, height) = img.dimensions();

    let mut down_img = ImageBuffer::new(width/two, height/two);
	let mut temp_img = ImageBuffer::new(width, height);
	temp_img = copy_from_gi(img);

	for n in 0..lvl {
		println!("n={:}",n);
		for x in 0..width/two.pow(n+1) {
			for y in 0..height/two.pow(n+1) {
				let px00 = temp_img.get_pixel(2*x, 2*y);
				let px01 = temp_img.get_pixel(2*x, 2*y+1);
				let px10 = temp_img.get_pixel(2*x+1, 2*y);
				let px11 = temp_img.get_pixel(2*x+1, 2*y+1);

				let mut tup; //= (k1, k2, k3, k4);
				tup = px00.channels4();
				let mut vec: (f64, f64, f64, f64) = (
				NumCast::from(tup.0).unwrap(),
				NumCast::from(tup.1).unwrap(),
				NumCast::from(tup.2).unwrap(),
				NumCast::from(tup.3).unwrap()
				);

				let mut r1 = vec.0;
				let mut g1 = vec.1;
				let mut b1 = vec.2;

				tup = px01.channels4();
				vec = (
				NumCast::from(tup.0).unwrap(),
				NumCast::from(tup.1).unwrap(),
				NumCast::from(tup.2).unwrap(),
				NumCast::from(tup.3).unwrap()
				);

				let mut r2 = vec.0;
				let mut g2 = vec.1;
				let mut b2 = vec.2;

				tup = px10.channels4();
				vec = (
				NumCast::from(tup.0).unwrap(),
				NumCast::from(tup.1).unwrap(),
				NumCast::from(tup.2).unwrap(),
				NumCast::from(tup.3).unwrap()
				);

				let mut r3 = vec.0;
				let mut g3 = vec.1;
				let mut b3 = vec.2;

				tup = px11.channels4();
				vec = (
				NumCast::from(tup.0).unwrap(),
				NumCast::from(tup.1).unwrap(),
				NumCast::from(tup.2).unwrap(),
				NumCast::from(tup.3).unwrap()
				);

				let mut r4 = vec.0;
				let mut g4 = vec.1;
				let mut b4 = vec.2;


				// let (r1, g1, b1) = get_rgb (px00);
				// let (r2, g2, b2) = get_rgb (px01);
				// let (r3, g3, b3) = get_rgb (px10);
				// let (r4, g4, b4) = get_rgb (px11);


            	let new_pixel = Pixel::from_channels(
                	NumCast::from(clamp((r1+r2+r3+r4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp((g1+g2+g3+g4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp((b1+b2+b3+b4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap(),
				);

				down_img.put_pixel(x, y, new_pixel);
			}
		}

		temp_img = down_img.clone();
	}

	let mut resize_img = ImageBuffer::new(width/two.pow(lvl), height/two.pow(lvl));
	for x in 0..width/two.pow(lvl) {
			for y in 0..height/two.pow(lvl) {
				let px = temp_img.get_pixel(x, y);
				
				let mut tup; //= (k1, k2, k3, k4);
				tup = px.channels4();
				let mut vec: (f64, f64, f64, f64) = (
				NumCast::from(tup.0).unwrap(),
				NumCast::from(tup.1).unwrap(),
				NumCast::from(tup.2).unwrap(),
				NumCast::from(tup.3).unwrap()
				);
				let mut r5 = vec.0;
				let mut g5 = vec.1;
				let mut b5 = vec.2;
				
				let new_pixel5 = Pixel::from_channels(
                	NumCast::from(clamp(r5, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp(g5, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp(b5, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap(),
				);

				resize_img.put_pixel(x, y, new_pixel5);

			}
	}
	//return
	resize_img
}
