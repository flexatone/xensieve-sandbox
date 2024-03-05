use image::Rgb;
use image::ImageBuffer;
use imageproc::drawing::draw_filled_circle;
// use imageproc::rect::Rect;
// use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
// fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
//     -> Result<(), std::io::Error>
// {
fn main() {

    let width = 100;
    let height = 100;

    let mut imgbuf = ImageBuffer::new(width, height);

    // Example: Fill the image with purple pixels
    // You can replace this part with your own logic to set pixels based on your array of color values
    for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
        // *pixel = Rgb([255u8, 0u8, 255u8]); // Purple
        if (x / 10) % 2 == 0 {
            *pixel = Rgb([255u8, 0u8, 0u8]); // Red for even stripes
        } else {
            *pixel = Rgb([0u8, 0u8, 255u8]); // Blue for odd stripes
        }
    }

    let circle_color = Rgb([255u8, 25u8, 12u8]);
    let _ = draw_filled_circle(&mut imgbuf, (50, 50), 100, circle_color);

    imgbuf.save("test-image.png").unwrap();
}
