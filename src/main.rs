use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::time::{Duration, Instant};

use video_rs::decode::Decoder;
use video_rs::location::Location;

use ndarray;

struct ImageBuffer<'a> {
    width: usize,
    height: usize,
    buffer: &'a [u8],
}

fn main() {
    video_rs::init().unwrap();

    let file = Location::File(PathBuf::from("vid/OpeningManim.mp4"));
    let mut decoder = Decoder::new(file).expect("Failed to create decoder");

    let mut frame_index = 0;
    let mut time_start = Instant::now();

    decoder
        .decode_iter()
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .for_each(|(_, frame)| {

        let rgb = frame.slice(ndarray::s![.., .., ..]).to_slice().unwrap();
        let (height, width, _) = frame.dim();

        let image_buffer = ImageBuffer {
            width,
            height,
            buffer: rgb,
        };

        write_image_buffer(image_buffer, frame_index).unwrap();
        frame_index += 1;

        let duration = time_start.elapsed();
        let fps = 1.0 / duration.as_secs_f32();
        println!("FPS: {}", fps);
        
        time_start = Instant::now();
    });
}

fn write_image_buffer(image_buffer: ImageBuffer, index: usize) -> std::io::Result<()> {
    let path = format!("out/debug{}.ppm", index);
    let header = format!("P6\n{} {} 255\n", image_buffer.width, image_buffer.height);

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.buffer)?;

    Ok(())
}
