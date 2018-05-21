
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;
use std::thread;
use std::sync::{Arc, Mutex};
static NTHREADS: i32 = 4;
extern crate image;

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
		let x_end = width / NTHREADS * (id+1);
		let radius = radius.clone();

		let child = thread::spawn(move || {
			let img = image::open("data/input.bmp").unwrap();
			let mut local_pixel_r = vec![0.0; _size];
			let mut local_pixel_g = vec![0.0; _size];
			let mut local_pixel_b = vec![0.0; _size];
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

					// (0, 0, 0) is black
					let mut _pos = (x * height + y) as usize;
					if _pos >= 192000 {
						_pos = 0;
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
		});

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