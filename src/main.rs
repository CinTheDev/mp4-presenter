use eframe::egui;

use std::time::Instant;

use ansi_term::Colour;

mod video_decoder;
use video_decoder::{VideoDecoder, VideoFrame};

const TARGET_FPS: f32 = 60.0;

fn main() {
    ffmpeg_next::init().unwrap();

    let decoder = VideoDecoder::new("vid/OpeningManim.mp4").expect("Failed to init decoder");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MP4-Presenter",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, decoder))),
    ).unwrap();
}

struct EguiApp {
    decoder: VideoDecoder,

    current_frame: Option<VideoFrame>,

    time_last_frame: Instant,
    target_time: std::time::Duration,
}

impl EguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        Self {
            decoder,
            current_frame: None,
            time_last_frame: Instant::now(),
            target_time: std::time::Duration::from_secs_f32(1.0 / TARGET_FPS),
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let delta_time = self.time_last_frame.elapsed();

        if delta_time < self.target_time {
            return;
        }

        self.current_frame = Some(self.decoder.get_frame());
        ctx.request_repaint();

        // FPS measuring
        let total_duration = self.time_last_frame.elapsed();

        let fps = 1.0 / total_duration.as_secs_f32();
        print_fps(fps);

        self.time_last_frame = Instant::now();
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.image(&self.image_texture);
        });

        self.update_texture(ctx);
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
