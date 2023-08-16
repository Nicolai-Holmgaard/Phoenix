// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;
mod config;

use file::{self as file_handler, get_photo_dir};
use screenshots::Screen;
// use tauri::{window, Window};
use std::fs;


#[tauri::command]
fn full_screenshot() {
    println!("Took a full screenshot");
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.to_png(None).unwrap();
        file_handler::save_file(buffer);
    }
} 

#[tauri::command]
fn area_screenshot(xstart: i32, ystart: i32, width: u32, height: u32) {
    println!("Took an area screenshot, width: {}, height: {}", width, height);
    let screen = Screen::from_point(xstart, ystart).unwrap();

    let image = screen.capture_area(xstart, ystart, width, height).unwrap();
    let buffer = image.to_png(None).unwrap();
    file_handler::save_file(buffer);
}


#[tauri::command]
async fn open_overlay(app_handle: tauri::AppHandle) {
    println!("You clicked");
    let screens = Screen::all().unwrap();
    
    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.to_png(None).unwrap();
        file_handler::save_file_named("temp", buffer);
    }
    
    
    let _docs_window = tauri::WindowBuilder::new(
        &app_handle,
        "overlay", /* the unique window label */
        tauri::WindowUrl::App("overlay.html".into()),
    ).build().unwrap().set_fullscreen(true);
    
}

#[tauri::command]
fn get_photos() -> Vec<String>{
    let mut photos: Vec<String> = vec![];
    let pathname = get_photo_dir();
    let test = fs::read_dir(pathname).unwrap();
    for path in test {
        let brah = String::from(path.unwrap().path().to_str().unwrap());
        photos.insert(photos.len(), brah);
    }
    photos
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![full_screenshot, area_screenshot, get_photos, open_overlay])
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            if event.window().label() == "main" {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        }
        _ => {},
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
