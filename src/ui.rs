use eframe::egui;
use ansi_term::Colour;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;

use crate::video_decoder::{VideoDecoder, VideoFrame};

pub const TARGET_FPS: f32 = 60.0;

pub struct EguiApp {
    video_rx: mpsc::Receiver<VideoFrame>,
    current_frame: Option<VideoFrame>,
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        let (video_tx, video_rx) = mpsc::channel();

        let ctx_thread = _cc.egui_ctx.clone();
        let target_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

        thread::spawn(move || {
            Self::receive_frames(decoder, video_tx, ctx_thread, target_time);
        });

        Self {
            video_rx,
            current_frame: None,
        }
    }

    fn update_frame(&mut self) {
        if let Ok(frame) = self.video_rx.try_recv() {
            self.current_frame = Some(frame);
        }
    }

    fn draw_frame(&mut self, ui: &mut egui::Ui) {
        if let Some(frame_wrapper) = self.current_frame.as_ref() {
            let frame = &frame_wrapper.frame;
        }
    }

    fn receive_frames(mut decoder: VideoDecoder, video_tx: mpsc::Sender<VideoFrame>, ctx: egui::Context, target_time: Duration) {
        let mut time_last_frame = Instant::now();

        while let Ok(frame) = decoder.get_frame() {
            if let Err(_) = video_tx.send(frame) {
                // Return if receiver is dropped
                return;
            }

            ctx.request_repaint();

            let work_time = time_last_frame.elapsed();

            // Wait so fps is consistent
            if work_time < target_time {
                let wait_time = target_time - work_time;
                thread::sleep(wait_time);
            }
            else {
                println!("{}", Colour::Yellow.bold().paint("BIG PROBLEM: LAG / BUFFER UNDERFLOW"));
            }

            // FPS measuring
            let total_duration = time_last_frame.elapsed();

            let fps = 1.0 / total_duration.as_secs_f32();
            print_fps(fps);

            time_last_frame = Instant::now();
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_frame(ui);
        });

        self.update_frame();
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
