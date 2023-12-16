// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn haha(text: &str) -> String{
    format!("hahaha,{}！", text)
}

// mod core;
// use crate::core::read_file::*;

// fn read_file()->std::io::Result<String>{
//     let mut reader = MyBufReader::open("D:/OneDrive/桌面.西游记.txt")?;
//     let mut buffer = String::new();
//     let mut result = String::new();

//     while let Some(line) = reader.read_line(&mut buffer) {
//         // println!("{}", line?.trim());
//         result = line?.to_string();
//     }
//     println!("{}", result);
//     Ok(result)
// }

use std::fs::File;
use std::io::{SeekFrom, Seek, Read};

#[tauri::command]
fn get_text(indx: u64)-> String{
    let mut file = File::open("D:/Downloads/BrowserDownloads/文件-1GB.txt").expect("Failed to open file");
    println!("{:?}", file);
    // let mut handle = file.take(width);
    let _ = file.seek(SeekFrom::Start(indx));
    let mut buffer1 = [0; 10000];
    let _ = file.read_exact(&mut buffer1[..]);
    // let mut buffer = String::new();
    // let _ = file.read_to_string(&mut buffer);
    println!("{:?}",&buffer1[..10]);
    // buffer.iter().collect()
    String::from("haha")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,haha,get_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
