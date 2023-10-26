pub type Color = crate::vec3::Vec3;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (259.999 * pixel_color.x()) as i32,
        (259.999 * pixel_color.y()) as i32,
        (259.999 * pixel_color.z()) as i32
    );
}
