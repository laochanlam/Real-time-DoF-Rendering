
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;

pub fn count_coc <I: GenericImage> (img: &I) -> Vec<u8> 
    where I::Pixel: 'static {
    
    let (width, height) = img.dimensions();
    let mut a: Vec<u8> = vec![0; NumCast::from(width * height).unwrap()];

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let (k1, _, _, _) = px.channels4();

            let k1_unwrap: f64 = NumCast::from(k1).unwrap();
            let mut radius: u32 = (k1_unwrap + 1.0).sqrt() as u32;
            if radius % 2 == 0 {
                radius += 1;
            }
            // a.push(NumCast::from(y).unwrap());
			a[((x as u32)*height + y) as usize] = radius as u8;
            // assert_eq!(a[a.len()-1], 0);
        }
    }

    // return
    a
}

pub fn redden <I: GenericImage> (img: &I)                            // first line is template and parameter 
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>    // second line is return value
    where I::Pixel: 'static {                                       // third line (and after) is where clause

    let (width, height) = img.dimensions();
    let mut img_out = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let (k1, k2, k3, k4) = px.channels4();
            let vec: (f64, f64, f64, f64) = (
                NumCast::from(k1).unwrap(),
                NumCast::from(k2).unwrap(),
                NumCast::from(k3).unwrap(),
                NumCast::from(k4).unwrap()
            );

            let r = vec.0 + 100.0;
            let g = vec.1;
            let b = vec.2;
            let wtf = vec.3;

            let new_pixel = Pixel::from_channels(
                NumCast::from(clamp(r, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(g, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(b, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(wtf, 0.0, 255.0)).unwrap()
            );

            img_out.put_pixel(x, y, new_pixel);
        }
    }
    
    // return
    img_out
}

pub fn whatever <I: GenericImage> (img: &I, radius: &mut Vec<u8>)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static {

	let (width, height) = img.dimensions();
	let (width, height) = (width as i32, height as i32);
	let mut img_out = ImageBuffer::new(width as u32, height as u32);
	const _size: usize = 1600*1200 + 1;
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
			let _pos = x * height + y;
			let r: i32 = radius[_pos as usize] as i32;
			let x_left: i32 = x - (r-1)/2;
			let x_right: i32 = x + (r-1)/2;
			let y_left: i32 = y - (r-1)/2;
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

			let new_pixel = Pixel::from_channels(
                NumCast::from(clamp(r, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(g, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(b, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap()
            );

            img_out.put_pixel(x as u32, y as u32, new_pixel);
		}
	}

	// return 
	img_out
}