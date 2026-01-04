use kmeans::*;
use image::GenericImageView;
use std::path::PathBuf;
use std::fs;
use std::env;

fn load_and_flatten_image(path: &PathBuf) -> (Vec<f32>, usize) {
   let img = image::open(path).expect("Failed to load image");

   let small_img = img.thumbnail(200, 200);

   let (width, height) = small_img.dimensions();
   let sample_cnt = (width * height) as usize;

   let pixels: Vec<f32> = small_img.to_rgb8()
       .pixels()
       .flat_map(|p| vec![p[0] as f32, p[1] as f32, p[2] as f32])
       .collect();
    
    (pixels, sample_cnt)
}

fn get_wallpaper_colors(path: &PathBuf) -> Vec<[f32; 3]>{
    let k = 5;
    let (pixel_data, sample_cnt) = load_and_flatten_image(path);

    let kmean: KMeans<f32, 8, _> = KMeans::new(
        &pixel_data, 
        sample_cnt, 
        3, 
        kmeans::EuclideanDistance
    );

    let result = kmean.kmeans_lloyd(
        k,
        100,
        KMeans::init_kmeanplusplus,
        &KMeansConfig::default()
    );

    let mut prominent_colors: Vec<[f32; 3]> = Vec::with_capacity(k);

    for i in 0..k {
        let color_slice = &result.centroids[i];
        
        let r = color_slice[0];
        let g = color_slice[1];
        let b = color_slice[2];
        
        prominent_colors.push([r, g, b]);
    }

    prominent_colors
}

fn get_luminance(color: &[f32; 3]) -> f32 {
    (0.2126 * color[0]) + (0.7152 * color[1]) + (0.0722 * color[2])
}

fn format_rgba(color: [f32; 3], alpha: f32) -> String {
    format!("rgba({}, {}, {}, {})", 
        color[0].round() as u8, 
        color[1].round() as u8, 
        color[2].round() as u8,
        alpha
    )
}

fn print_color(color: [f32; 3]) {
    let r = color[0] as u8;
    let g = color[1] as u8;
    let b = color[2] as u8;
    
    print!("\x1b[48;2;{};{};{}m       \x1b[0m ", r, g, b);
}

fn generate_theme(colors: &mut Vec<[f32; 3]>) -> ([f32; 3], [f32; 3], [f32; 3]) {
    colors.sort_by(|a, b| {
        get_luminance(a).partial_cmp(&get_luminance(b)).unwrap()
    });

    let bg_color = colors[0];
    let highlight_color = colors[4]; // Assuming k=5
    let font_color = colors[3];

    println!("--- Theme Generated ---");
    print_color(bg_color);
    println!("Background: {:?}", bg_color);
    print_color(font_color);
    println!("Font Color: {:?}", font_color);
    print_color(highlight_color);
    println!("Highlight:  {:?}", highlight_color);

    (bg_color, highlight_color, font_color)
}

pub fn update_waybar_theme(bg: [f32; 3], hl: [f32; 3], font: [f32; 3]) {
    let home = env::var("HOME").expect("Could not find $HOME");
    let path_to_style = format!("{}/.config/waybar/style.css", home);

    let bg_rgba = format_rgba(bg, 0.8); 
    let font_rgba = format_rgba(font, 1.0);
    let hl_rgba = format_rgba(hl, 1.0);

    let current_content = fs::read_to_string(&path_to_style)
        .expect("Couldn't read the Waybar style file.");

    let new_lines: Vec<String> = current_content.lines().map(|line| {
        let trimmed = line.trim();

        if trimmed.starts_with("@define-color bg_color") {
            format!("@define-color bg_color {};", bg_rgba)
        } 
        else if trimmed.starts_with("@define-color text_color") {
            format!("@define-color text_color {};", font_rgba)
        } 
        else if trimmed.starts_with("@define-color highlight_color") {
            format!("@define-color highlight_color {};", hl_rgba)
        } 
        else {
            line.to_string()
        }
    }).collect();

    fs::write(&path_to_style, new_lines.join("\n"))
        .expect("Failed to write to Waybar style file");

    let _ = std::process::Command::new("pkill")
        .arg("-USR2")
        .arg("waybar")
        .spawn();
}

pub fn change_theme(path: &PathBuf) {
    let mut colors = get_wallpaper_colors(path);
    let (bg, hl, font) = generate_theme(&mut colors);
    update_waybar_theme(bg, hl, font);
} 


