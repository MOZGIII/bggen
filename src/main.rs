use std::env;
use std::fs::File;
use std::io::BufWriter;

use anyhow::anyhow;

fn scale_one(value: usize, limit: usize) -> u8 {
    (value * 255 / limit) as u8
}

fn scale(x: usize, y: usize, width: usize, height: usize) -> [u8; 4] {
    let x = scale_one(x, width);
    let y = scale_one(y, height);
    [x, y, 255 / 2 + 255 / 4, 255]
}

fn genimage(width: usize, height: usize) -> Vec<u8> {
    let mut image = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            image.extend(scale(x, y, width, height));
        }
    }
    image
}

fn main() -> Result<(), anyhow::Error> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Expected a filename to output to."))?;

    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let (width, height) = (4000, 4000 / 4 * 3);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = genimage(width as usize, height as usize);
    writer.write_image_data(data.as_ref())?;

    Ok(())
}
