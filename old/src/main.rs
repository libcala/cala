extern crate dirs;
extern crate inflector;

use inflector::Inflector;
use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::fs;

fn get_name() -> String {
    let mut file = fs::File::open("Cargo.toml").unwrap();
    let mut string = String::new();

    file.read_to_string(&mut string).unwrap();

    let lines = string.lines();

    for line in lines {
        if line.starts_with("name = \"") {
            return line[8..(line.len() - 1)].to_string();
        }
    }
    panic!("Failed to find package name");
}

fn main() {
    // Iterpret arguments.
    let arg = if let Some(arg) = std::env::args().skip(1).next() {
        arg
    } else {
        eprintln!("Usage:\n\n    \
            siyo update   # Update siyo.\n    \
            siyo dive     # Run in Dive environment.\n    \
            siyo run      # Run natively.\n    \
            siyo flatpak  # Build a Flatpak.\n    \
            siyo windows  # Build a Windows installer EXE.\n    \
            siyo dmg      # Build a MacOS package.\n    \
            siyo apk      # Build an Android package & Deploy to device.\n    \
            siyo web      # Build a Web app & serve it (using cargo web).\n    \
            siyo site     # Build a static Web app (slow, use to publish only).\n    \
            siyo switch   # Build for Nintendo Switch.\n    \
        ");
        std::process::exit(255);
    };
    match arg.as_str() {
        "update" => {
            println!("Updating….");
            Command::new("cargo")
                .arg("install")
                .arg("siyo")
                .arg("--force")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("Failed to update");
        }
        "dive" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "run" => {
            let name = get_name();
            println!("Running \"{}\" ({})…", name.to_title_case(), name);
            fs::create_dir_all("app").unwrap();
            if cfg!(target_os = "linux") {
                println!("Generate Linux (XCB) Bindings…");
            } else {
                eprintln!("TODO");
                std::process::exit(255);
            }
        }
        "flatpak" => {
            eprintln!("TODO: Get code from cargo-gsp GitHub Repository.");
            std::process::exit(255);
        }
        "windows" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "dmg" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "apk" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "web" => {
            // Generate index.html
            let name = get_name();
            fs::create_dir_all("static").unwrap();
            let string = format!("<!doctype html><head><meta charset=\"utf-8\"><title>{}</title><style>\
                html,body,canvas{{margin:0px;padding:0px;width:100%;height:100%;overflow:hidden;}}</style>\
                </head><body><canvas id=\"canvas\"></canvas><script src=\"{}.js\"></script></body>\
                </html>",
                name.to_title_case(), name);
            let mut file = fs::File::create("static/index.html").unwrap();
            file.write_all(string.as_bytes()).unwrap();

            // Run cargo web.
            Command::new("cargo")
                .arg("web")
                .arg("start")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("failed to start server");
        }
        "site" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "switch" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        command => {
            eprintln!("Unknown command {}", command);
            std::process::exit(255);
        }
    }
}
