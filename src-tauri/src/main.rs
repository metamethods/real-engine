// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use serde::Serialize;

#[derive(Serialize)]
struct ProcessData {
    pid: u32,
    name: String
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_processes() -> Vec<ProcessData> {
    let system = System::new_all();

    system.processes()
        .iter()
        .map(|(pid, process)| ProcessData {
            pid: pid.as_u32(),
            name: process.name().to_string()
        })
        .collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}