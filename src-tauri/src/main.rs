// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashSet, time::Instant};

use device_query::{DeviceQuery, DeviceState, Keycode};
use screenshots::{
    image::{ImageBuffer, Rgba},
    Screen,
};
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct KeyPayload {
    code: String,
}

#[derive(Clone, serde::Serialize)]
struct MouseMovePayload {
    coords: (i32, i32),
}

#[derive(Clone, serde::Serialize)]
struct MouseIdlePayload {
    coords: (i32, i32),
}

#[derive(Clone, serde::Serialize)]
struct ScreenBrightnessPayload {
    brightness: (f64, f64, f64),
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

fn emit_mouse_idle_event(handle: &AppHandle, coords: &(i32, i32)) {
    handle
        .emit_all("mouse_idle", MouseIdlePayload { coords: *coords })
        .unwrap();
}

fn emit_screen_brightness_event(handle: &AppHandle, brightness: &(f64, f64, f64)) {
    handle
        .emit_all("screen_brightness", ScreenBrightnessPayload { brightness: *brightness })
        .unwrap();
}

// this function is written by ChatGPT, says: "thanks ChatGPT!"
fn analyze_brightness(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> (f64, f64, f64) {
    let (width, height) = image.dimensions();
    let mut total_brightness = 0u64;
    let mut brightness_values = Vec::with_capacity((width * height) as usize);

    for pixel in image.pixels() {
        // 将 RGBA 转换为灰度值
        let gray_value =
            (0.299 * pixel[0] as f64 + 0.587 * pixel[1] as f64 + 0.114 * pixel[2] as f64) as u8;
        total_brightness += gray_value as u64;
        brightness_values.push(gray_value);
    }

    let pixel_count = (width * height) as f64;
    let mean_brightness = total_brightness as f64 / pixel_count;

    // 计算中值亮度
    brightness_values.sort();
    let median_brightness = if brightness_values.len() % 2 == 0 {
        (brightness_values[brightness_values.len() / 2 - 1] as f64
            + brightness_values[brightness_values.len() / 2] as f64)
            / 2.0
    } else {
        brightness_values[brightness_values.len() / 2] as f64
    };

    // 计算标准差
    let std_brightness = (brightness_values
        .iter()
        .map(|&val| {
            let diff = val as f64 - mean_brightness;
            diff * diff
        })
        .sum::<f64>()
        / pixel_count)
        .sqrt();

    (mean_brightness, median_brightness, std_brightness)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.app_handle();
            let handle_clone = handle.clone();

            let mut last_mouse_coords = (0, 0);
            let mut key_down: HashSet<Keycode> = HashSet::new();

            let mut last_capture_time = Instant::now();

            std::thread::spawn(move || loop {
                let device_state = DeviceState::new();
                let mouse = device_state.get_mouse();
                let keys = device_state.get_keys();

                if last_mouse_coords != mouse.coords {
                    emit_mouse_move_event(&handle, &mouse.coords);
                    last_mouse_coords = mouse.coords;
                } else {
                    emit_mouse_idle_event(&handle, &mouse.coords);
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

            std::thread::spawn(move || loop {
                let now = Instant::now();
                if now.duration_since(last_capture_time).as_millis() >= 100 {
                    let screens = Screen::all().unwrap();
                    for screen in screens {
                        let image = screen.capture().unwrap();
                        emit_screen_brightness_event(&handle_clone, &analyze_brightness(&image));
                    }
                    last_capture_time = now
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
