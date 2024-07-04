use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::{mpsc, Mutex};

use crate::video_decoder::VideoDecoder;

const IMG_BUFFER: usize = 256;

#[derive(Component)]
struct Player {
    frame_rx: Mutex<mpsc::Receiver<Vec<u8>>>,
    image_handle: Handle<Image>,

    file_list: Vec<String>,
    animation_index: usize,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_next_frame,
            check_input,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
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

    // Image Bundle
    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: image_handle.clone().into(),
        ..default()
    });

    // Player
    let files = get_all_files("vid");

    let frame_rx = create_decoder(&files[0]);

    commands.spawn(Player {
        frame_rx: Mutex::new(frame_rx),
        image_handle,

        file_list: files,
        animation_index: 0,
    });
}

fn player_next_frame(
    player_query: Query<&Player>,
    mut images: ResMut<Assets<Image>>,
) {
    let player = player_query.single();
    let frame_rx = player.frame_rx.lock().unwrap();

    let receive_frame = frame_rx.recv();

    if receive_frame.is_err() {
        return
    }

    let frame_data = receive_frame.unwrap();
    let image = images.get_mut(&player.image_handle).unwrap();
    image.data = frame_data;
}

fn check_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::ArrowLeft => {
                load_next_video(player_query.single_mut().as_mut(), -1);
            },
            KeyCode::ArrowRight => {
                load_next_video(player_query.single_mut().as_mut(), 1);
            },

            _ => (),
        }
    }
}

fn load_next_video(player: &mut Player, index_offset: isize) {
    let animation_index = player.animation_index as isize + index_offset;
    let max_allowed = player.file_list.len() as isize - 1;

    // Don't reload when new index is out of bounds
    if animation_index < 0 || animation_index > max_allowed {
        return
    }

    player.animation_index = animation_index as usize;
    let next_file = &player.file_list[player.animation_index];

    let frame_rx = create_decoder(next_file);
    player.frame_rx = Mutex::new(frame_rx);
}

// --------------------
// | DECODING RELATED |
// --------------------

fn create_decoder(path: &str) -> mpsc::Receiver<Vec<u8>> {
    let decoder = VideoDecoder::new(path).unwrap();

    let (tx, rx) = mpsc::sync_channel(IMG_BUFFER);

    let task_pool = AsyncComputeTaskPool::get();
    task_pool.spawn(async move {
        decode(decoder, tx);
    }).detach();

    rx
}

fn decode(mut decoder: VideoDecoder, tx: mpsc::SyncSender<Vec<u8>>) {
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
