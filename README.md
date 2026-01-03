paperchange-rs

A fast, interactive CLI wallpaper manager specifically built for hyprpaper.

This utility automates the tedious process of updating hyprpaper.conf. Instead of manually changing file paths and reloading the daemon, you can pick a new look directly from your terminal.
🛠 Required Setup

For this tool to function, your Hyprland configuration must follow this specific structure:

    Configuration File: You must have a hyprpaper.conf located at ~/.config/hypr/hyprpaper.conf.

    Wallpaper Directory: Your images must be stored in a folder named wallpapers inside your hypr directory: ~/.config/hypr/wallpapers/.

Directory Map
Plaintext

~/.config/hypr/
├── hyprpaper.conf          <-- The tool modifies this
└── wallpapers/             <-- Put your .jpg and .png files here
    ├── forest.jpq
    ├── synthwave.png
    └── ...

✨ How it Works

When you run the tool, it:

    Scans your ~/.config/hypr/wallpapers/ directory.

    Displays an indexed list of available images.

    Rewrites the preload and wallpaper lines in your hyprpaper.conf.

    Restarts hyprpaper automatically so the changes take effect instantly.

🚀 Installation & Usage
Build
Bash

cargo build --release

Run
Bash

./target/release/your_binary_name

Controls

    Enter a Number: Sets that specific wallpaper.

    Type 'r': Sets a random wallpaper from your folder.

⚠️ Important Note

This tool rewrites lines in your hyprpaper.conf that start with preload and wallpaper. If you have a highly complex multi-monitor setup with different wallpapers on every screen, ensure you back up your config before the first run to see how the tool handles your specific layout.

Would you like me to add a "Troubleshooting" section to the README in case the killall command fails for users who don't have it installed?

yeah sure

Great. Adding a troubleshooting section and a keybinding tip makes the repo much more "user-ready." It anticipates the common hurdles and makes the tool feel like a native part of the Hyprland experience.

Here is the finalized README.md with the directory requirements, troubleshooting, and a bonus section on integration.
HyprPaper-RS 🦀

A lightweight, interactive CLI wallpaper manager specifically built for hyprpaper on Hyprland.

This utility automates the process of updating your hyprpaper.conf. Instead of manually editing file paths, use this terminal interface to browse, select, or randomize your desktop background.
🛠 Required Setup

For this tool to function, your Hyprland configuration must follow this structure:

    Config File: ~/.config/hypr/hyprpaper.conf must exist.

    Wallpaper Folder: Your images must be in ~/.config/hypr/wallpapers/.

Directory Map
Plaintext

~/.config/hypr/
├── hyprpaper.conf          <-- Tool updates this file
└── wallpapers/             <-- .jpg and .png files go here
    ├── sunrise.jpg
    ├── mountain.png
    └── city_night.jpg

✨ Features

    Live Config Injection: Automatically rewrites preload and wallpaper lines.

    Randomizer: Hit r for a random selection from your collection.

    Instant Refresh: Kills and restarts the hyprpaper daemon automatically via hyprctl.

    Rust Powered: Extremely fast and minimal resource footprint.

🚀 Installation & Usage
1. Build
Bash

cargo build --release

2. Run
Bash

./target/release/hyprpaper-selector

3. Keybindings (Optional)

To make it easily accessible, add this to your hyprland.conf to open the selector in a floating terminal (assuming kitty is your terminal):
Ini, TOML

bind = $mainMod, W, exec, kitty --class floating_selector -e /path/to/your/binary
windowrule = float, title:^(floating_selector)$

🔍 Troubleshooting

killall not found The tool uses killall to stop the existing hyprpaper instance. If your distro doesn't have it, install psmisc:

    Arch: sudo pacman -S psmisc

    Fedora: sudo dnf install psmisc

    Ubuntu/Debian: sudo apt install psmisc

Config not updating? Ensure your hyprpaper.conf isn't write-protected and that the lines you want replaced actually start with the words preload or wallpaper.
