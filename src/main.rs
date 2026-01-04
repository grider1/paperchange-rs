use std::fs;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::ffi::OsStr;

pub mod theme_changer;

fn main() {
    let home = env::var("HOME").expect("Could not find $HOME");
    let path_to_wallpapers = format!("{}/.config/hypr/wallpapers/", home);

    let mut wallpaper_list: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(&path_to_wallpapers) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && (path.extension().expect("no extension") == OsStr::new("jpg") || path.extension().expect("no extension") == OsStr::new("png")) {
                wallpaper_list.push(path);
            }
        }
    }

    println!("HI! Here are your wallpapers...\n");

    for (i, path) in wallpaper_list.iter().enumerate() {
        let wallpaper_name = path.file_name().unwrap_or_default().to_string_lossy(); 
        
        println!("{}. {}", i, wallpaper_name);
    }
    println!("r. Random Wallpaper"); 

    print!("Enter number to select: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    match input.trim().parse::<char>() {
        Ok(character) => {
            if character == 'r' {
                let random_index = rand::rng().random_range(0..wallpaper_list.len());
                let selected_path = &wallpaper_list[random_index];
                theme_changer::change_theme(selected_path);
                update_config(&selected_path);
                restart_hyprpaper();
                return;
            } 
        }
        Err(_) => println!("Invalid input: Please enter a number."),
    }
    
    match input.trim().parse::<usize>() {
        Ok(num) => {
            if num < wallpaper_list.len() {
                let selected_path = &wallpaper_list[num];
                theme_changer::change_theme(selected_path);
                update_config(&selected_path);
                restart_hyprpaper();
            }
        }
        Err(_) => println!("Invalid input: Please enter a number."),
    }
}

fn update_config(wallpaper_path: &PathBuf) {
    let home = env::var("HOME").expect("Could not find $HOME");
    let path_to_hyprpaper_config = format!("{}/.config/hypr/hyprpaper.conf", home);

    let current_content = fs::read_to_string(&path_to_hyprpaper_config)
        .expect("Couldn't read the config file.");

    let new_wallpaper_str = wallpaper_path.to_string_lossy();

    let new_lines: Vec<String> = current_content.lines().map(|line| {
        let trimmed = line.trim();

        if trimmed.starts_with("preload") {
            format!("preload = {}", new_wallpaper_str)
        } 
        else if trimmed.starts_with("wallpaper") {
            if let Some((monitor_part, _)) = line.split_once(',') {
                format!("{},{}", monitor_part, new_wallpaper_str)
            } else {
                format!("wallpaper = ,{}", new_wallpaper_str)
            }
        } 
        else {
            line.to_string()
        }
    }).collect();

    fs::write(&path_to_hyprpaper_config, new_lines.join("\n"))
        .expect("Failed to write to config file");
}

fn restart_hyprpaper() {
    let _ = Command::new("killall").arg("hyprpaper").status();
    thread::sleep(Duration::from_millis(500));

    Command::new("hyprctl").arg("dispatch").arg("exec").arg("hyprpaper").stdout(Stdio::null()).stderr(Stdio::null()).spawn().expect("failed to start hyprpaper");
}
