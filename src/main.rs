mod linux;
mod windows;
use dirs::home_dir;
use std::fs;

fn find_python() {
    // get os name
    let os = std::env::consts::OS;
    // println!("OS: {}", os);
    match os {
        "windows" => windows::find_python_windows(),
        "linux" => linux::find_python_linux(),
        &_ => todo!(),
    }
}
fn main() {
    let home = home_dir().unwrap();
    let path = home.join(".pvm");
    match fs::create_dir(&path) {
        Ok(_) => println!("Created .pvm directory in home directory"),
        Err(_) => println!("Directory already exists"),
    }
    find_python();
}
