
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;

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

	for x in 0..width {
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
			let _pos = (x * height + y) as usize;
			let r: i32 = radius[_pos];
			let x_left:  i32 = x - (r-1)/2;
			let x_right: i32 = x + (r-1)/2;
			let y_left:  i32 = y - (r-1)/2;
			let y_right: i32 = y + (r-1)/2;

			for i in x_left..x_right {
				for j in y_left..y_right {
					if i<0 || i>= width || j<0 || j>= height {
						continue;
					}

					pixel_r[(i*height + j) as usize] += tup.0 / ((r*r) as f64);
					pixel_g[(i*height + j) as usize] += tup.1 / ((r*r) as f64);
					pixel_b[(i*height + j) as usize] += tup.2 / ((r*r) as f64);
				}
			}
		}
	}

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