use image::{GrayImage, Luma, DynamicImage};

use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;


fn compute_energy_map(img: &GrayImage) -> Vec<Vec<u32>> {
    let (width, height) = img.dimensions();
    let mut energy = vec![vec![0; (width - 2) as usize]; (height - 2) as usize]; // Reduced dimensions

    let sobel_x: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];


    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut gx = 0;
            let mut gy = 0;

            for dy in 0..3 {
                for dx in 0..3 {
                    let px = img.get_pixel((x + dx - 1) as u32, (y + dy - 1) as u32).0[0] as i32;
                    gx += px * sobel_x[dy as usize][dx as usize];
                    gy += px * sobel_y[dy as usize][dx as usize];
                }
            }

            energy[(y - 1) as usize][(x - 1) as usize] = ((gx * gx + gy * gy) as f64).sqrt() as u32;
        }
    }

    energy
}


fn measure_time<F: FnOnce() -> T, T>(f: F) -> (T, std::time::Duration) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration) 
}



fn compute_energy_map2(img: &GrayImage) -> Vec<Vec<u32>> {
    let (width, height) = img.dimensions();
    let energy = Arc::new(Mutex::new(vec![vec![0; (width - 2) as usize]; (height - 2) as usize])); // Reduced dimensions

    let sobel_x: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

 
    (1..height - 1).into_par_iter().for_each(|y| {
  
        for x in 1..width - 1 {
            let mut gx = 0;
            let mut gy = 0;

 
            for dy in 0..3 {
                for dx in 0..3 {
                    let px = img.get_pixel((x + dx - 1) as u32, (y + dy - 1) as u32).0[0] as i32;
                    gx += px * sobel_x[dy as usize][dx as usize];
                    gy += px * sobel_y[dy as usize][dx as usize];
                }
            }

            let mut energy = energy.lock().unwrap();
            energy[(y - 1) as usize][(x - 1) as usize] = ((gx * gx + gy * gy) as f64).sqrt() as u32;
        }
    });

    Arc::try_unwrap(energy).unwrap().into_inner().unwrap()
}










fn main() {
    
    let img = image::open("/Users/kitipholk/Desktop/bigben.jpeg")
        .expect("Failed to open image")
        .into_luma8();

    let energy_map = compute_energy_map(&img);


    let width = img.width() as usize;
    let height = img.height() as usize;

println!("Initial first 3 rows (grayscale values):");
for y in 0..3.min(height) {
    let row: Vec<u8> = (0..width).map(|x| img.get_pixel(x as u32, y as u32).0[0]).collect();
    println!("{:?}", row);
}

println!("Initial last 3 rows (grayscale values):");
for y in height.saturating_sub(3)..height {
    let row: Vec<u8> = (0..width).map(|x| img.get_pixel(x as u32, y as u32).0[0]).collect();
    println!("{:?}", row);
}




    println!("The first row {:?}", energy_map.get(0));
    println!("the second row {:?}", energy_map.get(1));
    println!("the third row{:?}", energy_map.get(2));
    println!("the fourth row{:?}", energy_map.get(3));
    println!("the second last row{:?}", energy_map.get(energy_map.len() - 2));
    println!("the last{:?}", energy_map.get(energy_map.len() - 1));

    // // Print energy values for debugging
    // for row in &energy_map {
    //     println!("{:?}", row);
    // }
    let (_energy_map1, duration1) = measure_time(|| compute_energy_map(&img));

    // Measure and compare the time taken by compute_energy_map2
    let (_energy_map2, duration2) = measure_time(|| compute_energy_map2(&img));

    // Print out the time differences
    println!("Time taken by compute_energy_map: {:?}", duration1);
    println!("Time taken by compute_energy_map2: {:?}", duration2);
}
