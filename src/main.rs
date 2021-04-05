

use cgmath::Vector3;

mod ray;

fn main() {

    let v = Vector3::new(1,0,0);

    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (r * 255.0) as u8;
            let ig = (g * 255.0) as u8;
            let ib = (b * 255.0) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    let ray = ray::Ray{origin:Vector3::new(0,0,0), direction:Vector3::new(0,0,-1)};

}
