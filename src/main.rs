use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::time::Instant;

use video_rs::decode::Decoder;
use video_rs::location::Location;

use ndarray;

use ansi_term::Colour;

struct ImageBuffer<'a> {
    width: usize,
    height: usize,
    buffer: &'a [u8],
}

fn main() {
    video_rs::init().unwrap();

    let file = Location::File(PathBuf::from("vid/OpeningManim.mp4"));
    let decoder_reader = Decoder::new(file).expect("Failed to create decoder");

    //let mut frame_index = 0;
    let mut time_start = Instant::now();

    let (mut decoder, mut reader, stream_index) = decoder_reader.into_parts();

    // TODO: Decode whole video
    for _ in 0..30 {
        // Read
        let packet = reader.read(stream_index).unwrap();

        // Decode (fps drop expected - it means it's actually doing work)
        let _ = decoder.decode(packet).expect("Decode error");

        // FPS measuring
        let duration = time_start.elapsed();
        let fps = 1.0 / duration.as_secs_f32();
        print_fps(fps);

        time_start = Instant::now();
    }

    /*
    decoder
        .decode_raw_iter()
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .for_each(|frame| {

        //let rgb = frame.slice(ndarray::s![.., .., ..]).to_slice().unwrap();
        //let (height, width, _) = frame.dim();

        //let image_buffer = ImageBuffer {
        //    width,
        //    height,
        //    buffer: rgb,
        //};

        //write_image_buffer(image_buffer, frame_index).unwrap();
        //frame_index += 1;

        let duration = time_start.elapsed();
        let fps = 1.0 / duration.as_secs_f32();
        print_fps(fps);

        time_start = Instant::now();
    });
    */
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
