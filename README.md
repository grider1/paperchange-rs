# paperchange-rs 🦀

A lightweight, interactive CLI wallpaper and theme manager specifically built for **hyprpaper** and **Waybar** on Hyprland.

Instead of manually editing configuration files and restarting daemons, **paperchange-rs** provides a fast terminal interface to browse, select, or randomize your desktop background by managing your `hyprpaper.conf` automatically.

---

## ✨ Features

* **Interactive Terminal UI:** Lists all available wallpapers with index numbers for easy selection.
* **Dynamic Waybar Theming:** Automatically extracts prominent colors from your new wallpaper and updates global variables in your Waybar `style.css`.
* **Randomized Mode:** Type `r` to instantly pick a random wallpaper from your collection.
* **Automatic Config Patching:** Automatically finds and updates `preload` and `wallpaper` lines in your `hyprpaper.conf`.
* **Smart Refresh:** Handles the `killall`, `hyprctl`, and Waybar reload logic for you, ensuring the change happens instantly without leaving the terminal.
* **Native Rust Performance:** Compiled for speed with minimal dependencies.

---

## 🛠 Required Setup

For this tool to function, your configuration **must** follow this specific structure:

1.  **Hyprpaper Config:** A `hyprpaper.conf` must exist at `~/.config/hypr/hyprpaper.conf`.
2.  **Waybar Style:** A `style.css` must exist at `~/.config/waybar/style.css`.
3.  **Wallpaper Folder:** Your images must be stored in a folder named `wallpapers` inside your hypr directory: `~/.config/hypr/wallpapers/`.

### Directory Map
```text
~/.config/
├── hypr/
│   ├── hyprpaper.conf      <-- Tool updates this file
│   └── wallpapers/         <-- .jpg and .png files go here
│       ├── forest.jpg
│       └── ocean.png
└── waybar/
    └── style.css           <-- Tool updates global variables here
```

### Waybar CSS Variables
To enable the theme changer, you must add these specific global variables to the top of your `~/.config/waybar/style.css`. The tool targets these lines to apply the new color palette:

```css
@define-color bg_color rgba(33, 28, 43, 0.8);
@define-color text_color rgba(96, 78, 95, 1);
@define-color highlight_color rgba(164, 135, 111, 1);
```
