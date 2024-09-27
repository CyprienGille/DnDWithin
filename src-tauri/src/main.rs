// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use character::{Character, CharacterState};
use tauri::State;

mod ability;
mod character;
mod commons;
mod description;
mod equipment;
mod feature;
mod spell;

#[tauri::command]
fn get_character(character_state: State<CharacterState>) -> Character {
    (*character_state.state.lock().unwrap()).clone()
}

fn main() {
    tauri::Builder::default()
        .manage(CharacterState::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
