// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, vec};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_folder(fp: &str) -> Vec<String>{
    let mut ret = Vec::new();
    for file in fs::read_dir(fp).unwrap(){
        let path = file.unwrap().path();
        let path_str = path.to_str();
        ret.push(String::from(path_str.unwrap()));
    }
    ret
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
