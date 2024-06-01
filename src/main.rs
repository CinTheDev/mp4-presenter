use eframe;

mod video_decoder;
use video_decoder::VideoDecoder;

mod ui;
use ui::EguiApp;

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
