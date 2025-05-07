use tauri::{App, Manager};

use std::sync::Mutex;

enum Status {
    Rock,
    Paper,
    Scissors,
}
struct Test {
    status: Status,
}

impl Test {
    fn switch(&mut self) {
        match self.status {
            Status::Rock => self.status = Status::Scissors,
            Status::Scissors => self.status = Status::Paper,
            Status::Paper => self.status = Status::Rock,
        }
    }
}


pub mod model;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn shit(name: &str) -> String {
    format!("Hi There, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_list() -> Vec<String> {
    let mut result = vec![];

    result.push("A".to_string());
    result.push("B".to_string());
    result.push("C".to_string());
    result.push("D".to_string());
    result.push("E".to_string());
    result.push("F".to_string());


    result
}

#[tauri::command]
fn change_state(app_handle: tauri::AppHandle) -> Result<(), String> {

    let app_state = app_handle.state::<Mutex<Test>>();

    let mut state = app_state.lock().map_err(|e| e.to_string())?;

    state.switch();
    Ok(())
}

#[tauri::command]
fn get_state(app_handle: tauri::AppHandle) -> Result<String, String> {

    let app_state = app_handle.state::<Mutex<Test>>();

    let state = app_state.lock().map_err(|e| e.to_string())?;

    let status_string = match state.status {
        Status::Rock => "Rock",
        Status::Paper => "Paper",
        Status::Scissors => "Scissors",
    };

    Ok(status_string.to_string())

}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Test {status: Status::Rock}));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![shit, change_state, get_state, get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
