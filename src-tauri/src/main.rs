// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;

use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct KeyPayload {
    code: String,
}

#[derive(Clone, serde::Serialize)]
struct MouseMovePayload {
    coords: (i32, i32),
}

fn emit_key_down_event(handle: &AppHandle, code: String) {
    handle
        .emit_all("key_down", KeyPayload { code: code })
        .unwrap();
}

fn emit_key_up_event(handle: &AppHandle, code: String) {
    handle
        .emit_all("key_up", KeyPayload { code: code })
        .unwrap();
}

fn emit_mouse_move_event(handle: &AppHandle, coords: &(i32, i32)) {
    handle
        .emit_all("mouse_move", MouseMovePayload { coords: *coords })
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.app_handle();

            let mut last_mouse_coords = (0, 0);
            let mut key_down: HashSet<Keycode> = HashSet::new();

            std::thread::spawn(move || loop {
                let device_state = DeviceState::new();
                let mouse = device_state.get_mouse();
                let keys = device_state.get_keys();

                if last_mouse_coords != mouse.coords {
                    emit_mouse_move_event(&handle, &mouse.coords);
                    last_mouse_coords = mouse.coords;
                }

                for k in &keys {
                    if !key_down.contains(k) {
                        key_down.insert(*k);
                        emit_key_down_event(&handle, k.to_string());
                    }
                }
                key_down.retain(|&k| {
                    if !keys.contains(&k) {
                        emit_key_up_event(&handle, k.to_string());
                        return false;
                    }
                    true
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
