use std::fs::File;
use std::io::prelude::*;

use eframe::egui;

use std::time::Instant;

use ffmpeg_next::util::frame::video::Video;

use ansi_term::Colour;

mod video_decoder;
use video_decoder::VideoDecoder;

const TARGET_FPS: f32 = 60.0;
use std::thread::sleep;

fn main() {
    ffmpeg_next::init().unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MP4-Presenter",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    ).unwrap();

    /*
    let mut decoder = VideoDecoder::new("vid/OpeningManim.mp4").expect("Failed to init decoder");

    let mut frame_index = 0;
    let mut total_time_start = Instant::now();
    let mut work_start = Instant::now();

    let total_time = std::time::Duration::from_secs_f32(1.0 / TARGET_FPS);

    while let Ok(image_buffer) = decoder.get_frame() {
        write_image_buffer(&image_buffer, frame_index).expect("Failed to write image buffer");

        let work_duration = work_start.elapsed();
        
        // Wait so fps becomes constant
        if work_duration < total_time {
            let wait_time = total_time - work_duration;
            sleep(wait_time);
        }
        else {
            println!("BIG PROBLEM: BUFFER UNDERFLOW");
        }
        
        // FPS measuring
        let total_duration = total_time_start.elapsed();

        let fps = 1.0 / total_duration.as_secs_f32();
        print_fps(fps);

        frame_index += 1;

        total_time_start = Instant::now();
        work_start = Instant::now();
    }
    */
}

struct EguiApp {
    decoder: VideoDecoder,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        Self {
            decoder,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hi world");
        });
    }
}

fn print_fps(fps: f32) {
    let fps_status;

    let fps_deviance = (fps - TARGET_FPS).abs();

    if fps_deviance < 1.0 {
        fps_status = Colour::Green.paint("Pretty good");
    }
    else if fps_deviance < 5.0 {
        fps_status = Colour::Yellow.paint("Not great but ok");
    }
    else if fps_deviance < 10.0 {
        fps_status = Colour::Red.paint("Kinda bad");
    }
    else {
        fps_status = Colour::Red.bold().paint("Absolutely not consistent");
    }

    println!("FPS STATUS: {} ({:.1} fps)", fps_status, fps);
}

fn write_image_buffer(image_buffer: &Video, index: usize) -> std::io::Result<()> {
    let path = format!("out/debug{}.ppm", index);
    let header = format!("P6\n{} {} 255\n", image_buffer.width(), image_buffer.height());

    let mut file = File::create(path)?;

    file.write(header.as_bytes())?;
    file.write(&image_buffer.data(0))?;

    Ok(())
}
