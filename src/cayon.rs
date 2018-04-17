
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;

pub fn get_rgb <P: Pixel> (px: P)
	-> (f64, f64, f64) 
	where P::Subpixel: 'static {

	let (k1, k2, k3, k4) = px.channels4();
	let vec: (f64, f64, f64, f64) = (
		NumCast::from(k1).unwrap(),
		NumCast::from(k2).unwrap(),
		NumCast::from(k3).unwrap(),
		NumCast::from(k4).unwrap()
	);

	let mut r = vec.0;
	let mut g = vec.1;
	let mut b = vec.2;

	// return 
	(r,g,b)
}


pub fn downsize <I: GenericImage> (img: I, lvl: u32) 
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>   
    where I::Pixel: 'static {                                      
	let two: u32 =2;
    let (width, height) = img.dimensions();

    let mut down_img = ImageBuffer::new(width/two, height/two);

	for n in 0..lvl {
		for x in 0..width/2 {
			for y in 0..height/2 {
				let px00 = img.get_pixel(2*x, 2*y);
				let px01 = img.get_pixel(2*x, 2*y+1);
				let px10 = img.get_pixel(2*x+1, 2*y);
				let px11 = img.get_pixel(2*x+1, 2*y+1);

				let (r1, g1, b1) = get_rgb (px00);
				let (r2, g2, b2) = get_rgb (px01);
				let (r3, g3, b3) = get_rgb (px10);
				let (r4, g4, b4) = get_rgb (px11);


            	let new_pixel = Pixel::from_channels(
                	NumCast::from(clamp((r1+r2+r3+r4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp((g1+g2+g3+g4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp((b1+b2+b3+b4)/4.0, 0.0, 255.0)).unwrap(),
					NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap(),
				);

				down_img.put_pixel(x, y, new_pixel);
			}
		}
	}
  
	//return
	down_img
}
