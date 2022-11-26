#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Deserialize, Serialize, Debug, Default)]
struct Character {
    name: String,
    class_level: String,
    player_name: String,
    race: String,
    bg: String,
    exp: i64,
}

#[tauri::command]
fn get_default() -> Character {
    Character::default()
}

#[tauri::command]
async fn open_file() -> Character {
    let c = match FileDialogBuilder::new().pick_file() {
        Some(fp) => read_character_from_file(fp).unwrap_or_default(),
        None => Character::default(),
    };
    c
}

fn read_character_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}

fn main() {
    // println!("{:?}", read_character_from_file("./src/test.json"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_file, get_default])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
