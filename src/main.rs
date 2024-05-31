use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::time::Instant;

use ansi_term::Colour;

mod video_decoder;
use video_decoder::VideoDecoder;

struct ImageBuffer<'a> {
    width: usize,
    height: usize,
    buffer: &'a [u8],
}

fn main() {
    video_rs::init().unwrap();

    let path = PathBuf::from("vid/OpeningManim.mp4");
    let mut decoder = VideoDecoder::new(path).expect("Failed to read video file");
    decoder.start_decoding();

    let mut frame_index = 0;
    let mut time_start = Instant::now();

    // TODO: Decode whole video
    for _ in 0..30 {
        let frame_buffer = decoder.get_frame();

        let image_buffer = ImageBuffer {
            width: todo!(),
            height: todo!(),
            buffer: frame_buffer,
        };

        write_image_buffer(image_buffer, frame_index).expect("Failed to write image buffer");

        // FPS measuring
        let duration = time_start.elapsed();
        let fps = 1.0 / duration.as_secs_f32();
        print_fps(fps);

        time_start = Instant::now();
        frame_index += 1;
    }
}

fn print_fps(fps: f32) {
    let fps_status;
    if fps < 10.0 {
        fps_status = Colour::Red.bold().paint("Friggin terrible");
    }
    else if fps < 30.0 {
        fps_status = Colour::Red.paint("Garbage performance");
    }
    else if fps < 60.0 {
        fps_status = Colour::Yellow.bold().paint("Not good enough");
    }
    else if fps < 120.0 {
        fps_status = Colour::Yellow.paint("Could be better");
    }
    else {
        fps_status = Colour::Green.paint("Pretty good");
    }

    println!("FPS STATUS: {} ({:.2} fps)", fps_status, fps);
}

fn write_image_buffer(image_buffer: ImageBuffer, index: usize) -> std::io::Result<()> {
    let path = format!("out/debug{}.ppm", index);
    let header = format!("P6\n{} {} 255\n", image_buffer.width, image_buffer.height);

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.buffer)?;

    Ok(())
}
