use eframe::egui;

use std::time::Instant;

use ansi_term::Colour;

mod video_decoder;
use video_decoder::VideoDecoder;

const TARGET_FPS: f32 = 60.0;

use std::thread::sleep;

fn main() {
    ffmpeg_next::init().unwrap();

    let decoder = VideoDecoder::new("vid/OpeningManim.mp4").expect("Failed to init decoder");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MP4-Presenter",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, decoder))),
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

    image_texture: egui::TextureHandle,

    total_time_start: Instant,
    work_time_start: Instant,

    target_time: std::time::Duration,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>, decoder: VideoDecoder) -> Self {
        Self {
            decoder,
            image_texture: Self::instance_texture(&cc.egui_ctx),
            total_time_start: Instant::now(),
            work_time_start: Instant::now(),
            target_time: std::time::Duration::from_secs_f32(1.0 / TARGET_FPS),
        }
    }

    fn instance_texture(ctx: &egui::Context) -> egui::TextureHandle {
        // TODO: Consistent pixel count
        let default_raw_image: [u8; 1920 * 1080 * 3] = [0xff; 1920 * 1080 * 3];

        let raw_image = egui::ColorImage::from_rgb(
            [1920, 1080],
            &default_raw_image,
        );

        ctx.load_texture("Image", raw_image, egui::TextureOptions::default())
    }

    fn update_texture(&mut self) {
        let frame_response = self.decoder.get_frame();
        
        let texture_time_start = Instant::now();
        if let Ok(frame) = frame_response {

            // TODO: This operation copies the entire image into RAM which is like super slow,
            //       make it only store the reference so no massive copying happens
            let img = egui::ColorImage::from_rgb(
                [1920, 1080],
                frame.data(0),
            );
            
            println!("DEBUG: Texture duration: {:?}", texture_time_start.elapsed());
            self.image_texture.set(img, egui::TextureOptions::default());
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(&self.image_texture);
        });

        self.update_texture();
        let work_duration = self.work_time_start.elapsed();
        
        // Wait so fps becomes constant
        if work_duration < self.target_time {
            let wait_time = self.target_time - work_duration;
            sleep(wait_time);
        }
        else {
            println!("{}", Colour::Yellow.bold().paint("BIG PROBLEM: BUFFER UNDERFLOW / LAG"));
        }
        
        // FPS measuring
        let total_duration = self.total_time_start.elapsed();

        let fps = 1.0 / total_duration.as_secs_f32();
        print_fps(fps);

        self.total_time_start = Instant::now();
        self.work_time_start = Instant::now();

        ctx.request_repaint();
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
