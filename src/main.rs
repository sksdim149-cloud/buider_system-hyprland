use std::io::{self, Write};
use std::process::Command;

//helloworld("print")
fn hello() {
    clear_screen();
    println!("Hello, this is my system builder!\n");
}
// packages Install
fn install() {
    clear_screen();
    println!("Install my rice? (y/n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let answer = input.trim().to_lowercase();

    if answer == "y" {
        println!("Installing system packages...");

        let pacman_pkgs = [
            "kitty", "blueman", "bluez", "yay", "hyprland", "git", "curl",
            "firefox", "rofi", "linux-cachyos-headers", "xorg-xwayland",
            "qt5-wayland", "qt6-wayland", "nvidia-dkms", "nvidia-utils",
            "lib32-nvidia-utils", "egl-wayland", "yazi", "eza", "zsh",
            "nvim", "btop", "gdm", "cargo", "rust"
        ];

        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .args(&pacman_pkgs)
            .arg("--noconfirm")
            .status()
            .expect("Failed to execute pacman");

        if status.success() {
            println!("Installing AUR packages...");
            let yay_pkgs = ["zed", "waybar", "steam", "ayugram-desktop", "nekobox", "prism-launcher"];

            let yay_status = Command::new("yay")
                .arg("-S")
                .args(&yay_pkgs)
                .arg("--noconfirm")
                .status()
                .expect("Failed to execute yay");

            if yay_status.success() {
                clear_screen();
                println!("All done! Your rice is ready.");
            }
        }
    } else {
        println!("Aborting rice installation.");
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    hello();
    install();
}
