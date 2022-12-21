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
struct Spell {
    prep: String, // Is prepared
    name: String,
    save: String,
    time: String,
    range: String,
    comp: String, // Components
    duration: String,
    page_ref: String,
    notes: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Equipment {
    name: String,
    qty: i32,  // Quantity
    w: String, // Weight
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Atk {
    name: String,
    dmg: String,
    hit: String, //To hit bonus
    notes: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfOption {
    mult: f32,    //multiplicator (1.0 for prof, 2.0 for expertise...)
    text: String, // H for half, P for prof and E for expertise
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Character {
    name: String,
    class_level: String,
    player_name: String,
    race: String,
    bg: String, // Background
    exp: i64,   // Experience
    str: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
    prof_mod: i32,     // Proficiency Bonus
    prof_st_str: bool, // Proficiency in strength saving throws
    prof_st_dex: bool,
    prof_st_con: bool,
    prof_st_int: bool,
    prof_st_wis: bool,
    prof_st_cha: bool,
    st_mods: String,  // Saving Throws modifiers
    acro: ProfOption, // Abilities (ex: Acroobatics)
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
    ac: i32,     // Armor Class
    res: String, // Resistances and Immunities
    speeds: String,
    profs: String,
    max_hp: i32,
    hp: i32,
    temp_hp: i32,
    hd_total: String, // Total number of hit dice
    hd: String,       // Current number of hit dice
    ds_s_1: bool,     // Death saving throws - Success
    ds_s_2: bool,
    ds_s_3: bool,
    ds_f_1: bool, // Death saving throws - Failure
    ds_f_2: bool,
    ds_f_3: bool,
    actions: String,
    senses: String,
    atk_1: Atk,
    atk_2: Atk,
    atk_3: Atk,
    atk_4: Atk,
    atk_5: Atk,
    atk_6: Atk,
    features_1: String,
    features_2: String,
    features_3: String,
    cp: i32, // copper
    sp: i32, // silver
    ep: i32, // electrum
    gp: i32, // gold
    pp: i32, // platinum
    eq_1: Equipment,
    eq_2: Equipment,
    eq_3: Equipment,
    eq_4: Equipment,
    eq_5: Equipment,
    eq_6: Equipment,
    eq_7: Equipment,
    eq_8: Equipment,
    eq_9: Equipment,
    eq_10: Equipment,
    eq_11: Equipment,
    eq_12: Equipment,
    eq_13: Equipment,
    eq_14: Equipment,
    eq_15: Equipment,
    eq_16: Equipment,
    eq_17: Equipment,
    eq_18: Equipment,
    eq_19: Equipment,
    eq_20: Equipment,
    eq_21: Equipment,
    eq_22: Equipment,
    eq_23: Equipment,
    eq_24: Equipment,
    eq_25: Equipment,
    eq_26: Equipment,
    eq_27: Equipment,
    eq_28: Equipment,
    eq_29: Equipment,
    eq_30: Equipment,
    gender: String,
    age: String,
    size: String,
    height: String,
    weight: String,
    alig: String, // Alignment
    faith: String,
    skin: String,
    eyes: String,
    hair: String,
    appearance: String,
    allies: String,
    p_traits: String, // Personality traits
    ideals: String,
    bonds: String,
    flaws: String,
    backstory: String,
    notes_1: String,
    notes_2: String,
    sp_class: String, // Spellcasting class
    sp_ab: String,    // Spellcasting Ability
    sp_dc: String,    // Spell save DC
    sp_atk: String,   // spell attack bonus
    spells: Vec<Spell>,
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
