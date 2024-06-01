use eframe::egui;
use ansi_term::Colour;
use std::{sync::mpsc, time::{Duration, Instant}};

use crate::video_decoder::{VideoDecoder, VideoFrame};

pub const TARGET_FPS: f32 = 60.0;

pub struct EguiApp {
    decoder: VideoDecoder,

    video_rx: mpsc::Receiver<VideoFrame>,
    current_frame: Option<VideoFrame>,

    time_last_frame: Instant,
    target_time: std::time::Duration,
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        let (video_tx, video_rx) = mpsc::channel();

        Self {
            decoder,
            video_rx,
            current_frame: None,
            time_last_frame: Instant::now(),
            target_time: std::time::Duration::from_secs_f32(1.0 / TARGET_FPS),
        }
    }

    fn receive_frames(mut decoder: VideoDecoder, video_tx: mpsc::Sender<VideoFrame>, ctx: &egui::Context, target_time: Duration) {
        let mut time_last_frame = Instant::now();

        loop {
            let delta_time = time_last_frame.elapsed();
    
            if delta_time < target_time {
                return;
            }
    
            if let Ok(frame) = decoder.get_frame() {
                video_tx.send(frame).unwrap();
                ctx.request_repaint();
    
                // FPS measuring
                let total_duration = time_last_frame.elapsed();
    
                let fps = 1.0 / total_duration.as_secs_f32();
                print_fps(fps);
    
                time_last_frame = Instant::now();
            }
            else {
                return;
            }
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.image(&self.image_texture);
        });

        //self.update_texture(ctx);
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
