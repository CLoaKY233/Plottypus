use serde::Serialize;
use serialport::{available_ports};

#[derive(Serialize)]
struct SerialPortDetails {
    port_name: String,
    // Add any other fields you need from SerialPortInfo
}

#[tauri::command]
fn list_ports() -> Vec<SerialPortDetails> {
    let ports = available_ports().unwrap_or_else(|_| Vec::new());

    // Map SerialPortInfo to a custom struct with necessary details
    ports.iter().map(|port| SerialPortDetails {
        port_name: port.port_name.clone(),
        // Add other fields here if needed
    }).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_ports,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}