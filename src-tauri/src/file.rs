use std::fs;

pub fn save_file(img_buffer: Vec<u8>) -> String {
    let path = format!("{}{}.png",get_photo_dir(),"TestImg");

    fs::write(path.clone(), img_buffer).unwrap();
    return path
}


pub fn save_file_named(name: &str, img_buffer: Vec<u8>) -> String {
    let path = format!("{}{}.png",get_photo_dir(),name);

    fs::write(path.clone(), img_buffer).unwrap();
    return path
}

pub fn get_photo_dir() -> String {
    // ignore next two lines, i know they are ugly and very bad
    let home_dir = dirs::home_dir().unwrap(); // .to_str().unwrap();
    let home_dir_parsed = home_dir.to_str().unwrap();

    format!("{}/Pictures/Phoenix/", home_dir_parsed)
}