
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;


struct rgb{
	r: i32,
	g: i32,
	b: i32,
	wtf: i32
}


struct arrHh{
	arrH: [rgb; 1000000]
}

pub fn downsize <I: GenericImage> (img: I, lvl: u32) 
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>   
    where I::Pixel: 'static {                                      
	let two: u32 =2;
    let (width, height) = img.dimensions();
   // let mut down_img = ImageBuffer::new(width/(two.pow(lvl)), height/(two.pow(lvl)));

    let mut down_img = ImageBuffer::new(width, height);
        
	let arrW: [arrHh; 1000000]; 

	
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

            arrW[x/two].arrHh.arrH[y/two].rgb.r += vec.0;
            arrW[x/two].arrHh.arrH[y/two].rgb.g += vec.1;
            arrW[x/two].arrHh.arrH[y/two].rgb.b += vec.2;
            arrW[x/two].arrHh.arrH[y/two].rgb.wtf += vec.3;

			

/*            let new_pixel = Pixel::from_channels(
                NumCast::from(clamp(r, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(g, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(b, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(wtf, 0.0, 255.0)).unwrap()
			);
*/

		//	img_out.put_pixel(x, y, new_pixel);
		}
	}

	for x in 0..width/two {
        for y in 0..height/two {
			arrW[x].arrHh.arrH[y].rgb.r /= 4;
			arrW[x].arrHh.arrH[y].rgb.g /= 4;
			arrW[x].arrHh.arrH[y].rgb.b /= 4;
			arrW[x].arrHh.arrH[y].rgb.wtf /= 4;

            let new_pixel = Pixel::from_channels(
                NumCast::from(clamp(arrW[x].arrHh.arrH[y].rgb.r, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(arrW[x].arrHh.arrH[y].rgb.g, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(arrW[x].arrHh.arrH[y].rgb.b, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(arrW[x].arrHh.arrH[y].rgb.wtf, 0.0, 255.0)).unwrap()
			);
			down_img.put_pixel(x, y, new_pixel);
		}
	}
	//return
	down_img
	}
