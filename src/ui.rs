use eframe::egui;
use ansi_term::Colour;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;

use crate::video_decoder::VideoDecoder;

pub const TARGET_FPS: f32 = 60.0;

pub struct EguiApp {
    video_rx: mpsc::Receiver<egui::ColorImage>,
    image_texture: egui::TextureHandle,
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        let (video_tx, video_rx) = mpsc::channel();

        let ctx_thread = _cc.egui_ctx.clone();
        let target_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

        thread::spawn(move || {
            Self::receive_frames(decoder, video_tx, ctx_thread, target_time);
        });

        let default_image = egui::ColorImage::new(
            [1920, 1080],
            egui::Color32::RED,
        );
        let image_texture = _cc.egui_ctx.load_texture("Image", default_image, egui::TextureOptions::default());

        Self {
            video_rx,
            image_texture,
        }
    }

    fn update_frame(&mut self) {
        if let Ok(frame) = self.video_rx.try_recv() {
            //self.current_frame = Some(frame);
            self.image_texture.set(frame, egui::TextureOptions::default());
        }
    }

    fn draw_frame(&mut self, ui: &mut egui::Ui) {
        let sized_texture = egui::load::SizedTexture::from_handle(&self.image_texture);
        ui.image(sized_texture);
    }

    fn receive_frames(mut decoder: VideoDecoder, video_tx: mpsc::Sender<egui::ColorImage>, ctx: egui::Context, target_time: Duration) {
        let mut time_last_frame = Instant::now();

        while let Ok(frame) = decoder.get_frame() {
            let img = egui::ColorImage::from_rgb(
                [1920, 1080],
                frame.data(0),
            );
            if let Err(_) = video_tx.send(img) {
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
