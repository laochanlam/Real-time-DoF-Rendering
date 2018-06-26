extern crate image;
use image:: {GenericImage, ImageBuffer, Pixel};
use num::NumCast;
use num_traits::clamp;
use std::thread;
use std::sync::{Arc, Mutex};
static NTHREADS: i32 = 8;

pub fn get_dof <I: GenericImage> (img: &I) -> Vec<i32>
    where I::Pixel: 'static {

    let (width, height) = img.dimensions();
    let _size = NumCast::from(width * height).unwrap();
    let mut dof: Vec<i32> = vec![0; _size];
    let mut max = 0;
    let mut min = 255;

    for x in 0 .. width {
        for y in 0 .. height {
            let px = img.get_pixel(x, y);
            let (k1, _, _, _) = px.channels4();
            let k1: i32 = NumCast::from(k1).unwrap();

            if k1 > max { max = k1; }
            if k1 < min { min = k1; }

            dof[(x*height + y) as usize] = k1;
        }
    }

    let scale = ((max - min) as f64) / 255.0;
    // println!("scale = {}", scale);
    for i in 0.._size {
        // dof[i] = ((dof[i] as f64) / scale) as i32;

        if dof[i] == 0 {
            dof[i] = 255;
        }
    }

    // return
    dof
}

pub fn get_coc (dof: &mut Vec<i32>, ox: i32, oy: i32, width: i32, height: i32) -> Vec<i32> {
    
    let _size = NumCast::from(width * height).unwrap();
    let mut coc: Vec<i32> = vec![0; _size];
    let opos = ox * height + oy;
    let odof = dof[opos as usize];
    let mut min_c = 255;
    let mut max_c = 0; 

    for x in 0..width {
        for y in 0..height {
            let pos = (x*height + y) as usize;
            let mut dis = dof[pos] - odof;

            if dis < 0 {
                dis = dis * -1;
            }

            dis = ((dis+1) as f64).sqrt() as i32;
            if dis % 2 == 0 {
                dis = dis + 1;
            }

            coc[pos] = dis;

            if dis < min_c { min_c = dis; }
            if dis > max_c { max_c = dis; }
            // assert_eq!(a[a.len()-1], 0);
        }
    }

    println!("min_c = {}", min_c);
    println!("max_c = {}", max_c);

    // return
    coc
}

pub fn copy_from_gi<I: GenericImage>(img: &I)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>   
    where I::Pixel: 'static {

    let mut buf = ImageBuffer::new(img.width(), img.height());
    for x in 0 .. img.width() {
        for y in 0 .. img.height() {
            let p = img.get_pixel(x, y);
            buf.put_pixel(x, y, p);
        }
    }
    
    // return
    buf
}

#[allow(unused_mut)]    // for pixel_r
pub fn render <I: GenericImage> (img: &I, radius: &mut Vec<i32>, pathname: &String)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static {

    let (width, height) = img.dimensions();
    let (width, height) = (width as i32, height as i32);
    let _size = (width * height) as usize;
    let mut img_out = ImageBuffer::new(width as u32, height as u32);
    let pixel_r = vec![0.0; _size];
    let pixel_g = vec![0.0; _size];
    let pixel_b = vec![0.0; _size];
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
        let pathname = pathname.clone();

        let child = thread::spawn(move || {
            let mut local_pixel_r = vec![0.0; _size];
            let mut local_pixel_g = vec![0.0; _size];
            let mut local_pixel_b = vec![0.0; _size];
            let img   = image::open(pathname.clone() + "/ds1.png").unwrap();
            let img_g = image::open(pathname.clone() + "/ds2.png").unwrap();
            let img_d1= image::open(pathname.clone() + "/dst_downsize1.bmp").unwrap();
            let img_d2= image::open(pathname.clone() + "/dst_downsize2.bmp").unwrap();
            let img_d3= image::open(pathname.clone() + "/dst_downsize3.bmp").unwrap();	
            let img_d4= image::open(pathname.clone() + "/dst_downsize4.bmp").unwrap();
            // let img_d5= image::open("data/dst_downsize5.bmp".to_string()).unwrap();

            let (width_g, height_g) = img_g.dimensions();
            let (width_g, height_g) = (width_g as i32, height_g as i32);
            let mut distance_gray: Vec<i32> = vec![0; (width_g * height_g) as usize];
            for x in 0..width_g {
                for y in 0..height_g {
                    let px = img_g.get_pixel(x as u32, y as u32);
                    let (k1, _, _, _) = px.channels4();
                    let k1_unwrap: i32 = NumCast::from(k1).unwrap();
                    // TODO: make sure how to calculate mipmap level
                    distance_gray[(x*height_g+y) as usize] = k1_unwrap/60 + 1 as i32;
                }
            }

            for x in x_start..x_end {
                for y in 0..height {
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
                    let exp = (two.pow((d_lvl-1) as u32) as u32) as i32;			
                    let px = selected_img.get_pixel((x / exp) as u32, (y / exp)as u32);
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
                    let x_right: i32 = x + (r-1)/2 + 1;
                    let y_left:  i32 = y - (r-1)/2;
                    let y_right: i32 = y + (r-1)/2 + 1;

                    for i in x_left..x_right {
                        for j in y_left..y_right {
                            if i<0 || i>= width || j<0 || j>= height {
                                continue;
                            }
                            local_pixel_r[(i*height + j) as usize] += tup.0 / ((r*r) as f64);
                            local_pixel_g[(i*height + j) as usize] += tup.1 / ((r*r) as f64);
                            local_pixel_b[(i*height + j) as usize] += tup.2 / ((r*r) as f64);
                        }
                        // println!("id {}: end of a row", id);
                    }
                }
                // println!("hello: {}", x);
            }	// end of each pixel
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

#[allow(unused_assignments)]    // for temp_img
pub fn downsize <I: GenericImage> (img: &I, level: u32) 
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>   
    where I::Pixel: 'static {

    let two: u32 = 2;
    let (width, height) = img.dimensions();

    let mut down_img = ImageBuffer::new(width/two, height/two);
    let mut temp_img = ImageBuffer::new(width, height);
    temp_img = copy_from_gi(img);

    for n in 0..level {
        for x in 0..width/two.pow(n+1) {
            for y in 0..height/two.pow(n+1) {
                
                let px1 = temp_img.get_pixel(x*2, y*2);
                let px2 = temp_img.get_pixel(x*2, y*2+1);
                let px3 = temp_img.get_pixel(x*2+1, y*2);
                let px4 = temp_img.get_pixel(x*2+1, y*2+1);

                let px1 = px1.channels4();
                let px2 = px2.channels4();
                let px3 = px3.channels4();
                let px4 = px4.channels4();

                let r = px1.0 + px2.0 + px3.0 + px4.0;
                let g = px1.1 + px2.1 + px3.1 + px4.1;
                let b = px1.2 + px2.2 + px3.2 + px4.2;

                let r: f64 = NumCast::from(r).unwrap();
                let g: f64 = NumCast::from(g).unwrap();
                let b: f64 = NumCast::from(b).unwrap();

                let new_pixel = Pixel::from_channels(
                    NumCast::from(clamp(r/4.0, 0.0, 255.0)).unwrap(),
                    NumCast::from(clamp(g/4.0, 0.0, 255.0)).unwrap(),
                    NumCast::from(clamp(b/4.0, 0.0, 255.0)).unwrap(),
                    NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap(),
                );

                down_img.put_pixel(x, y, new_pixel);
            }
        }

        temp_img = down_img.clone();
    }

    let mut resize_img = ImageBuffer::new(width/two.pow(level), height/two.pow(level));
    for x in 0..width/two.pow(level) {
        for y in 0..height/two.pow(level) {
            let px = temp_img.get_pixel(x, y);
            let px = px.channels4();
            let px : (f64, f64, f64) = (
                NumCast::from(px.0).unwrap(),
                NumCast::from(px.1).unwrap(),
                NumCast::from(px.2).unwrap()
            );
            let px = Pixel::from_channels(
                NumCast::from(clamp(px.0, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(px.1, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(px.2, 0.0, 255.0)).unwrap(),
                NumCast::from(clamp(0.0, 0.0, 255.0)).unwrap(),
            );
            resize_img.put_pixel(x, y, px);
        }
    }

    // return
    resize_img
}
