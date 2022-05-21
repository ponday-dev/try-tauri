#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicI8, Ordering};
use serde::{Serialize, Deserialize};

struct CounterState(AtomicI8);

#[derive(Serialize, Deserialize)]
struct RequestMessage {
  name: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
  message: String,
}

#[tauri::command]
fn with_message(message: RequestMessage) -> ResponseMessage {
  ResponseMessage {
    message: format!("Hello, {}", message.name)
  }
}

#[tauri::command]
fn countup(state: tauri::State<CounterState>) -> i8 {
  state.0.fetch_add(1, Ordering::Relaxed) + 1
}

fn main() {
  tauri::Builder::default()
    .manage(CounterState(AtomicI8::new(0)))
    .invoke_handler(tauri::generate_handler![
      countup,
      with_message,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
