use std::fs::File;
use std::io::prelude::*;

use std::time::Instant;

use ffmpeg_next::util::frame::video::Video;

use ansi_term::Colour;

mod video_decoder;
use video_decoder::VideoDecoder;

const TARGET_FPS: f32 = 60.0;
use std::thread::sleep;

fn main() {
    ffmpeg_next::init().unwrap();

    let mut decoder = VideoDecoder::new("vid/OpeningManim.mp4").expect("Failed to init decoder");

    let mut frame_index = 0;
    let mut time_start = Instant::now();

    let total_time = std::time::Duration::from_secs_f32(1.0 / TARGET_FPS);

    while let Ok(image_buffer) = decoder.get_frame() {
        write_image_buffer(&image_buffer, frame_index).expect("Failed to write image buffer");

        // FPS measuring
        let duration = time_start.elapsed();
        time_start = Instant::now();

        let fps = 1.0 / duration.as_secs_f32();
        print_fps(fps);

        // Wait so fps becomes constant
        if duration < total_time {
            let wait_time = total_time - duration;
            sleep(wait_time);
        }

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

fn write_image_buffer(image_buffer: &Video, index: usize) -> std::io::Result<()> {
    let path = format!("out/debug{}.ppm", index);
    let header = format!("P6\n{} {} 255\n", image_buffer.width(), image_buffer.height());

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.data(0))?;

    Ok(())
}
