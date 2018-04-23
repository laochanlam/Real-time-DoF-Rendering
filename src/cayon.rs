
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;

pub fn get_rgb <P: Pixel>(px: P)
	-> (f64, f64, f64) 
	where P::Subpixel: 'static {

	// let (k1, k2, k3, k4) = px.channels4();
	// let vec: (f64, f64, f64, f64) = (
	// 	NumCast::from(k1).unwrap(),
	// 	NumCast::from(k2).unwrap(),
	// 	NumCast::from(k3).unwrap(),
	// 	NumCast::from(k4).unwrap()
	// );

	// let mut r = vec.0;
	// let mut g = vec.1;
	// let mut b = vec.2;

	// // return 
	// (r,g,b)

	(6.6,7.7,8.8)
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
		//println!("llll");
        buf
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
