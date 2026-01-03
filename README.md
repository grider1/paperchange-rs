# paperchange-rs 🦀

A lightweight, interactive CLI wallpaper manager specifically built for **hyprpaper** on Hyprland.

Instead of manually editing configuration files and restarting daemons, **paperchange-rs** provides a fast terminal interface to browse, select, or randomize your desktop background by managing your `hyprpaper.conf` automatically.

---

## ✨ Features

* **Interactive Terminal UI:** Lists all available wallpapers with index numbers for easy selection.
* **Randomized Mode:** Type `r` to instantly pick a random wallpaper from your collection.
* **Automatic Config Patching:** Automatically finds and updates `preload` and `wallpaper` lines in your `hyprpaper.conf`.
* **Smart Refresh:** Handles the `killall` and `hyprctl` reload logic for you, ensuring the change happens instantly without leaving the terminal.
* **Native Rust Performance:** Compiled for speed with minimal dependencies.

---

## 🛠 Required Setup

For this tool to function, your configuration **must** follow this specific structure:

1.  **Config File:** A `hyprpaper.conf` must exist at `~/.config/hypr/hyprpaper.conf`.
2.  **Wallpaper Folder:** Your images must be stored in a folder named `wallpapers` inside your hypr directory: `~/.config/hypr/wallpapers/`.

### Directory Map
```text
~/.config/hypr/
├── hyprpaper.conf          <-- Tool updates this file
└── wallpapers/             <-- .jpg and .png files go here
    ├── forest.jpg
    ├── ocean.png
    └── space.jpg
