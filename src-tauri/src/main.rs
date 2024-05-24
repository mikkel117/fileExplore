// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
//mod GetFile;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_standard_dirs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_standard_dirs() {
    let path = fs::read_dir("<path>").unwrap();
    println!("{:?}", path);
    let _metadata = fs::metadata("<path>").unwrap();
    for entry in path {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
