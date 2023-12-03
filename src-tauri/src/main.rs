// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::path::PathBuf;

use file_explorer_rust::FileInfo;
use simple_home_dir::home_dir;

#[tauri::command]
fn list_home() -> Result<PathBuf, String> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(String::from("Error while find files")),
    }
}

#[tauri::command]
fn list_dir(path: &str, show_hidden: bool) -> Result<Vec<FileInfo>, String> {
    let paths =
        file_explorer_rust::find_files(&path.parse::<PathBuf>().unwrap(), show_hidden).unwrap();

    match file_explorer_rust::find_files_info(&paths) {
        Ok(info) => Ok(info),
        Err(_) => Err(String::from("Error while find files")),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_home, list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
