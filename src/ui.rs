use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    println!("Bevy setup");
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
