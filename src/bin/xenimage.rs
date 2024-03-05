use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
// fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
//     -> Result<(), std::io::Error>
// {
fn main() {
    let pixels: &mut [u8] = [];
    let output = File::create("test-image.png")?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels,
                   800, 200,
                   ColorType::Gray(8))?;
    ()
}