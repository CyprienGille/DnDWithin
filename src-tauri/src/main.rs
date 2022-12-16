#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{write, File},
    io::BufReader,
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfOption {
    mult: f32,
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Character {
    name: String,
    class_level: String,
    player_name: String,
    race: String,
    bg: String,
    exp: i64,
    str: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
    prof_mod: i32,
    prof_st_str: bool,
    prof_st_dex: bool,
    prof_st_con: bool,
    prof_st_int: bool,
    prof_st_wis: bool,
    prof_st_cha: bool,
    st_mods: String,
    acro: ProfOption,
    anim: ProfOption,
    arca: ProfOption,
    athl: ProfOption,
    dece: ProfOption,
    hist: ProfOption,
    insi: ProfOption,
    inti: ProfOption,
    inve: ProfOption,
    medi: ProfOption,
    natu: ProfOption,
    perc: ProfOption,
    perf: ProfOption,
    pers: ProfOption,
    reli: ProfOption,
    slei: ProfOption,
    stea: ProfOption,
    surv: ProfOption,
    ac: i32,
    res: String,
    speeds: String,
    profs: String,
    max_hp: i32,
    hp: i32,
    temp_hp: i32,
    hd_total: String,
    hd: String,
    ds_s_1: bool,
    ds_s_2: bool,
    ds_s_3: bool,
    ds_f_1: bool,
    ds_f_2: bool,
    ds_f_3: bool,
    actions: String,
    senses: String,
    atk_name_1: String,
    atk_hit_1: String,
    atk_dmg_1: String,
    atk_notes_1: String,
    atk_name_2: String,
    atk_hit_2: String,
    atk_dmg_2: String,
    atk_notes_2: String,
    atk_name_3: String,
    atk_hit_3: String,
    atk_dmg_3: String,
    atk_notes_3: String,
    atk_name_4: String,
    atk_hit_4: String,
    atk_dmg_4: String,
    atk_notes_4: String,
    atk_name_5: String,
    atk_hit_5: String,
    atk_dmg_5: String,
    atk_notes_5: String,
    atk_name_6: String,
    atk_hit_6: String,
    atk_dmg_6: String,
    atk_notes_6: String,
}

#[tauri::command]
fn get_default() -> Character {
    Character {
        name: "Character name".to_string(),
        ..Default::default()
    }
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

#[tauri::command]
fn save_character_to_file(c: Character) {
    let path = match FileDialogBuilder::new().save_file() {
        Some(path) => path,
        None => PathBuf::from(""), // this ensures failure at the next step if the user closed the file dialog
    };
    println!(
        "Saving status : {:?}",
        write(path, serde_json::to_string_pretty(&c).unwrap())
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file,
            get_default,
            save_character_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
