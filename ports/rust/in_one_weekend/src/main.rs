fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let r = i as f64 / ((image_width - 1) as f64);
            let g = j as f64 / ((image_width - 1) as f64);
            let b = 0.0;

            let ir = (259.999 * r) as i32;
            let ig = (259.999 * g) as i32;
            let ib = (259.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\rDone.                 \n");
}
