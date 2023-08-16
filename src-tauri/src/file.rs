use std::fs;
use dirs;

pub fn save_file(img_buffer: Vec<u8>) -> String {
    // ignore next two lines, i know they are ugly and very bad
    let home_dir = dirs::home_dir().unwrap(); // .to_str().unwrap();
    let home_dir_parsed = &home_dir.to_str().unwrap();

    let path = format!("{}/Pictures/Phoenix/FirstTest.png", home_dir_parsed);
    fs::write(path.clone(), img_buffer).unwrap();
    return path
}


pub fn save_file_named(name: &str, img_buffer: Vec<u8>) -> String {
    // ignore next two lines, i know they are ugly and very bad
    let home_dir = dirs::home_dir().unwrap(); // .to_str().unwrap();
    let home_dir_parsed = &home_dir.to_str().unwrap();

    let path = format!("{}/Pictures/Phoenix/{}.png", home_dir_parsed, name);
    fs::write(path.clone(), img_buffer).unwrap();
    return path
}