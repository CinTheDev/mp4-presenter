use eframe::egui;
use ansi_term::Colour;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;
use std::vec::Vec;

use crate::video_decoder::VideoDecoder;

pub const TARGET_FPS: f32 = 60.0;

const IMAGE_BUFFER_SIZE: usize = 256;

pub struct EguiApp {
    frame_rx: Option<mpsc::Receiver<egui::ColorImage>>,
    image_texture: egui::TextureHandle,

    animation_sources: Vec<String>,
    animation_index: usize,
}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let default_image = egui::ColorImage::new(
            [1920, 1080],
            egui::Color32::RED,
        );
        let image_texture = cc.egui_ctx.load_texture("Image", default_image, egui::TextureOptions::default());

        let animation_sources = get_all_files("vid");

        Self {
            frame_rx: None,
            image_texture,
            animation_sources,
            animation_index: 0,
        }
    }

    fn update_frame(&mut self) {
        if let Ok(frame) = self.frame_rx.as_ref().unwrap().try_recv() {
            self.image_texture.set(frame, egui::TextureOptions::default());
        }
    }

    fn draw_frame(&mut self, ui: &mut egui::Ui) {
        let sized_texture = egui::load::SizedTexture::new(self.image_texture.id(), ui.available_rect_before_wrap().size());
        ui.image(sized_texture);
    }

    fn next_animation(&mut self, ctx: &egui::Context) {
        if self.animation_index + 1 < self.animation_sources.len() {
            self.animation_index += 1;
            self.reload_animation(ctx);
        }
    }

    fn previous_animation(&mut self, ctx: &egui::Context) {
        // If index is already 0 and 1 is subtracted, it will be the greatest integer,
        // in which case this check will fail regardless, so no casting required
        if self.animation_index - 1 < self.animation_sources.len() {
            self.animation_index -= 1;
            self.reload_animation(ctx);
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context, input: &egui::InputState) {
        for event in input.events.clone() {
            match event {
                egui::Event::Key {
                    key,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers: _,
                } => {
                    match key {
                        egui::Key::ArrowRight => {
                            self.next_animation(ctx);
                        }
                        egui::Key::ArrowLeft => {
                            self.previous_animation(ctx);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }

    fn reload_animation(&mut self, ctx: &egui::Context) {
        let (video_tx, video_rx) = mpsc::sync_channel(IMAGE_BUFFER_SIZE);
        let (frame_tx, frame_rx) = mpsc::channel();

        let source_path = self.animation_sources[self.animation_index].as_str();
        let decoder = VideoDecoder::new(source_path).expect("Failed to init decoder");
        
        thread::spawn(move || {
            Self::receive_frames(decoder, video_tx);
        });
        
        let ctx_thread = ctx.clone();
        let target_frame_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

        thread::spawn(move || {
            Self::receive_frames_timed(frame_tx, video_rx, ctx_thread, target_frame_time);
        });

        self.frame_rx = Some(frame_rx);
    }

    fn receive_frames_timed(
        frame_tx: mpsc::Sender<egui::ColorImage>,
        video_rx: mpsc::Receiver<egui::ColorImage>,
        ctx: egui::Context,
        target_frame_time: Duration,
    ) {
        let mut time_frame_start = Instant::now();

        loop {
            let received = video_rx.recv();
            if received.is_err() { return }

            let transmit_response = frame_tx.send(received.unwrap());
            if transmit_response.is_err() { return }

            ctx.request_repaint();

            let work_time = time_frame_start.elapsed();
            if work_time < target_frame_time {
                let wait_time = target_frame_time - work_time;
                thread::sleep(wait_time);
            }
            else {
                println!("{}", Colour::Yellow.bold().paint("BIG PROBLEM: LAG / BUFFER UNDERFLOW"));
            }

            // FPS measuring
            let total_duration = time_frame_start.elapsed();

            let fps = 1.0 / total_duration.as_secs_f32();
            //print_fps(fps);

            time_frame_start = Instant::now();
        }
    }

    fn receive_frames(mut decoder: VideoDecoder, video_tx: mpsc::SyncSender<egui::ColorImage>) {
        while let Ok(frame) = decoder.get_frame() {
            let img = egui::ColorImage::from_rgb(
                [1920, 1080],
                frame.data(0),
            );
            if let Err(_) = video_tx.send(img) {
                // Return if receiver is dropped
                return;
            }
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_frame(ui);
        });

        self.update_frame();
        
        ctx.input(|i| self.handle_input(ctx, i));
    }
}

fn get_all_files(dir: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let dir_entries = std::fs::read_dir(dir).expect("Cannot open animation source directory");

    for read_entry in dir_entries {
        let entry = read_entry.expect("Cannot read animation source file");
        let path = entry
            .path()
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        
        println!("Animation source: {}", path);
        result.push(path);
    }

    result
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
