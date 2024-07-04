mod video_decoder;
mod ui;

fn main() {
    ffmpeg_next::init().unwrap();

    ui::run();
}
