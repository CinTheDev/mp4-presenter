use bevy::prelude::*;

use crate::video_decoder::VideoDecoder;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(commands: Commands) {
    let files = get_all_files("vid");

    create_player(commands, &files[0]);
}

fn create_player(mut commands: Commands, path: &str) {
    commands.spawn(Player {
        path: VideoDecoder::new(path),
    });
}

#[derive(Component)]
struct Player {
    decoder: VideoDecoder,
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
        
        result.push(path);
    }

    result.sort_unstable();

    result
}
