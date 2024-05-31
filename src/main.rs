use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct ImageBuffer {
    width: usize,
    height: usize,
    buffer: Box<[u8]>,
}

fn main() {
    let f = File::open("vid/OpeningManim.mp4").unwrap();
    let size = f.metadata().unwrap().len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();

    println!("Timescale: {}", mp4.moov.mvhd.timescale);
    println!("Size: {}", mp4.size());
    println!("Duration: {:?}", mp4.duration());
}

fn write_image_buffer(image_buffer: ImageBuffer) -> std::io::Result<()> {
    let path = "debug.ppm";
    let header = format!("P6\n{} {} 255\n", image_buffer.width, image_buffer.height);

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.buffer)?;

    Ok(())
}
