
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;

pub fn redden <I: GenericImage> (img: I)                            // first line is template and parameter 
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