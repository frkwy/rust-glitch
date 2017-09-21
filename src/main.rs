#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate image;
extern crate rand;
use std::env;
use std::fs::File;
use rand::Rng;
use std::path::Path;

fn clamp(val: i32, min: i32, max: i32) -> i32 {
    if val <= min { return min; }
    if val >= max { return max; }
    val
}

fn main() {
    let filename = if env::args().len() == 2 {
        let args: Vec<_> = env::args().collect();
        args[1].clone()
    } else {
        panic!("Please pass a filename.")
    };
    
    let mut buf = image::open(&Path::new(&filename)).unwrap().to_rgb();
    println!("dimensions {:?}", buf.dimensions());
    
    let (w, h) = buf.dimensions();
    let mut xoff: i32 = 0;
    let mut yoff: i32 = 0;
    let mut rng = rand::thread_rng();
    
    for y in 0..h {
        for x in 0..w {
            if rand::random::<u16>() < 100 {
                xoff += rng.gen_range(-1 as i32, 2);
            }
            if rand::random::<u16>() < 500 {
                yoff += rng.gen_range(-1 as i32, 2);
            }
            if rand::random::<u16>() < 10 {
                xoff /= 2;
                yoff /= 2;
            }
            
            let srcx = clamp(((x as i32) + xoff), 0, (w - 1) as i32);
            let srcy = clamp(((y as i32) + yoff), 0, (h - 1) as i32);
            let srcpx = buf[(srcx as u32, srcy as u32)];

            buf.put_pixel(x, y, srcpx);
        }
    }
    
    let out_filename = format!("{}.rg.png", filename);
    let fout = &mut File::create(&Path::new(&out_filename)).unwrap();
    let _ = image::ImageRgb8(buf).save(fout, image::PNG);
    println!("Saved => {0}", out_filename);
}
