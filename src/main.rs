#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// App state to track interactions
#[derive(Default)]
struct AppState {
    click_count: Mutex<i32>,
    messages: Mutex<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct MessageResponse {
    echo: String,
    count: i32,
    timestamp: String,
}

#[tauri::command]
fn button_clicked(state: State<AppState>) -> String {
    let mut count = state.click_count.lock().unwrap();
    *count += 1;

    println!(
        "🚀 Button was clicked from the frontend! (Click #{})",
        *count
    );

    match *count {
        1 => "Hello from Rust backend! 👋".to_string(),
        2..=5 => format!("Welcome back! Click #{}", *count),
        6..=10 => format!("You're getting the hang of this! 🎉 ({})", *count),
        _ => format!("Wow, {} clicks! You really like this button! 🔥", *count),
    }
}

#[tauri::command]
fn send_message(message: String, state: State<AppState>) -> MessageResponse {
    println!("📨 Received message from frontend: '{}'", message);

    // Store the message
    state.messages.lock().unwrap().push(message.clone());

    // Get current counts
    let click_count = *state.click_count.lock().unwrap();
    let message_count = state.messages.lock().unwrap().len() as i32;

    // Generate response based on message content
    let echo = match message.to_lowercase().as_str() {
        msg if msg.contains("hello") => "Hello there! 👋 Rust says hi back!".to_string(),
        msg if msg.contains("tauri") => "Tauri is awesome for desktop apps! 🚀".to_string(),
        msg if msg.contains("rust") => "Rust is blazingly fast and memory safe! 🦀".to_string(),
        msg if msg.contains("help") => {
            "I'm here to help! Try asking about Rust or Tauri!".to_string()
        }
        _ => format!("I received: '{}' - Thanks for the message!", message),
    };

    MessageResponse {
        echo,
        count: message_count,
        timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
    }
}

#[tauri::command]
fn get_stats(state: State<AppState>) -> serde_json::Value {
    let clicks = *state.click_count.lock().unwrap();
    let messages = state.messages.lock().unwrap().len();

    println!(
        "📊 Stats requested - Clicks: {}, Messages: {}",
        clicks, messages
    );

    serde_json::json!({
        "total_clicks": clicks,
        "total_messages": messages,
        "uptime": "Session active",
        "status": "running"
    })
}

fn main() {
    println!("🚀 Starting Tauri + Tailwind App...");
    println!("📝 All frontend interactions will be logged here!");
    println!("───────────────────────────────────────────────");

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            button_clicked,
            send_message,
            get_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
