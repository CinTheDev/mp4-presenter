use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::{mpsc, Mutex};

use crate::video_decoder::VideoDecoder;

const IMG_BUFFER: usize = 256;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_next_frame)
        .run();
}

fn setup(mut commands: Commands) {
    let files = get_all_files("vid");

    let frame_rx = create_player(&files[0]);
    commands.insert_resource(CurrentPlayer {
        player: Mutex::new(frame_rx),
    });
}

#[derive(Resource)]
struct CurrentPlayer {
    player: Mutex<mpsc::Receiver<Vec<u8>>>,
}

fn player_next_frame(
    current_player_res: Res<CurrentPlayer>,
) {
    let player = current_player_res.player.lock().unwrap();

    let receive_frame = player.recv();

    if receive_frame.is_err() {
        return
    }

    let frame_data = receive_frame.unwrap();
    println!("Frame data aquired!");
    drop(frame_data);
}

fn create_player(path: &str) -> mpsc::Receiver<Vec<u8>> {
    let player = Player {
        decoder: VideoDecoder::new(path).unwrap(),
    };

    let (tx, rx) = mpsc::sync_channel(IMG_BUFFER);

    let task_pool = AsyncComputeTaskPool::get();
    task_pool.spawn(async move {
        run_player(player, tx);
    }).detach();

    rx
}

struct Player {
    decoder: VideoDecoder,
}

fn run_player(mut player: Player, tx: mpsc::SyncSender<Vec<u8>>) {
    while let Ok(frame) = player.decoder.get_frame() {
        let frame_vec = Vec::from(frame.data(0));

        if let Err(_) = tx.send(frame_vec) {
            return;
        }
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
        
        result.push(path);
    }

    result.sort_unstable();

    result
}
