use eframe;

mod video_decoder;

mod ui;
use ui::EguiApp;

fn main() {
    ffmpeg_next::init().unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MP4-Presenter",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    ).unwrap();
}
