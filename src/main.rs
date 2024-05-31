use std::fs::File;
use std::io::prelude::*;

use std::time::Instant;

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;

use ansi_term::Colour;

//mod video_decoder;
//use video_decoder::VideoDecoder;

/*
struct ImageBuffer {
    width: usize,
    height: usize,
    buffer: Box<[u8]>,
}
*/

fn main() {
    ffmpeg_next::init().unwrap();

    //let path = PathBuf::from("vid/OpeningManim.mp4");
    //let mut decoder = VideoDecoder::new(path).expect("Failed to read video file");

    let mut video_file = input("vid/OpeningManim.mp4").unwrap();
    let input = video_file
        .streams()
        .best(Type::Video)
        .unwrap();
    let video_stream_index = input.index();

    let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    ).unwrap();
    
    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                write_image_buffer(&rgb_frame, frame_index).unwrap();
                frame_index += 1;
            }

            Ok(())
        };
    
    for (stream, packet) in video_file.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
    }

    decoder.send_eof().unwrap();
    receive_and_process_decoded_frames(&mut decoder).unwrap();

    /*
    //let (width, height) = decoder.get_dimensions();

    //decoder.start_decoding();

    let mut time_start = Instant::now();

    // TODO: Decode whole video
    for _ in 0..30 {
        let frame_buffer = decoder.get_frame();

        let image_buffer = ImageBuffer {
            width,
            height,
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

fn write_image_buffer(image_buffer: &Video, index: usize) -> std::io::Result<()> {
    let path = format!("out/debug{}.ppm", index);
    let header = format!("P6\n{} {} 255\n", image_buffer.width(), image_buffer.height());

    println!("Writing frame {}", index);

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.data(0))?;

    Ok(())
}
