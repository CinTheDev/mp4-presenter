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

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Player
    let files = get_all_files("vid");

    let frame_rx = create_player(&files[0]);

    // Image
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
    use bevy::render::render_asset::RenderAssetUsages;
    let mut image = Image::new_fill(
        Extent3d {
            width: 1920,
            height: 1080,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0xFF, 0x00, 0x00, 0xFF],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;

    let image_handle = images.add(image);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Image
    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: image_handle.clone().into(),
        ..default()
    });

    commands.spawn(CurrentPlayer {
        player: Mutex::new(frame_rx),
        image_handle
    });
}

#[derive(Component)]
struct CurrentPlayer {
    player: Mutex<mpsc::Receiver<Vec<u8>>>,
    image_handle: Handle<Image>,
}

fn player_next_frame(
    current_player_query: Query<&CurrentPlayer>,
    mut images: ResMut<Assets<Image>>,
) {
    let current_player = current_player_query.single();
    let player = current_player.player.lock().unwrap();

    let receive_frame = player.recv();

    if receive_frame.is_err() {
        return
    }

    let frame_data = receive_frame.unwrap();
    let image = images.get_mut(&current_player.image_handle).unwrap();
    image.data = frame_data;
}

fn create_player(path: &str) -> mpsc::Receiver<Vec<u8>> {
    let decoder = VideoDecoder::new(path).unwrap();

    let (tx, rx) = mpsc::sync_channel(IMG_BUFFER);

    let task_pool = AsyncComputeTaskPool::get();
    task_pool.spawn(async move {
        run_player(decoder, tx);
    }).detach();

    rx
}

fn run_player(mut decoder: VideoDecoder, tx: mpsc::SyncSender<Vec<u8>>) {
    while let Ok(frame) = decoder.get_frame() {
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
