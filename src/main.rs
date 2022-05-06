use image::{ImageBuffer, Rgb}; 
use qd::qd;

fn get_samples(r: f64, count: i32, warmup: i32) -> Vec::<f64> {
    // Given r, sample that r value some number of times
    
    let mut start = 0.5f64;

    for _ in 0..warmup {
        start = r * start * (1f64 - start);
    }

    let mut samples = Vec::<f64>::new();

    for y in 0..count { 
        start = r * start * (1f64 - start);

        samples.push(start);
    }

    samples
}

fn main() {
    const imgx: usize = 3000;
    const imgy: usize = 2000;

    let xmin = 3.855981f64;
    let xmax = 3.856505f64;
    let ymin=0f64;
    let ymax=0.8f64;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::<image::Rgb::<u8>, Vec::<_>>::new(imgx as u32, imgy as u32);
    let mut counters = Vec::<[u32; imgx as usize]>::with_capacity(imgy as usize);

    for i in 0..imgy {
        counters.push([0u32; imgx as usize]);
    }
    

    let xsamples = 10000;

    for pxunr in 0..xsamples {
        if pxunr % 10000 == 0 {
            println!("{}", pxunr);
        }

        let x = pxunr as f64 / xsamples as f64 * (xmax - xmin) + xmin;
        
        let px = (pxunr as f64 / xsamples as f64 * imgx as f64) as usize;
        let samples = get_samples(x, 500, 300);

        let mut count = [0u32; imgy as usize];
        for f in &samples {
            let idx = (f * (imgy as f64)) as usize;
            if idx >= 0 && idx < imgy {
                count[idx] = count[idx].saturating_add(1);
            }
        }

        for py in 0usize..imgy {
            let factor = count[py] * imgy as u32;

            counters[py][px] += factor;
        }
    }

    let maxV = counters.iter().map(|a| a.iter().max().unwrap()).max().unwrap();

    for px in 0usize..imgx {
        for py in 0usize..imgy {
            let v: u8 = (counters[py][px] as f64 / (*maxV as f64) * 255. * 128.) as u8;
            *imgbuf.get_pixel_mut(px as u32, py as u32) = image::Rgb([v, v, v]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal2.png").unwrap();
}
