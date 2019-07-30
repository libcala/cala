extern crate dirs;
extern crate inflector;

use inflector::Inflector;
use std::io::{Write, Read};
use std::process::{Command, Stdio};
use std::fs;
use std::path;

fn get_emsdk(siyo_dir: &path::PathBuf) -> path::PathBuf {
    let mut emsdk_dir = siyo_dir.clone();
    emsdk_dir.push("emsdk-master");

    if emsdk_dir.exists() == false {
        let mut emsdk_zip = siyo_dir.clone();
        emsdk_zip.push("emsdk.zip");

        Command::new("wget")
            .arg("-O")
            .arg(&emsdk_zip)
            .arg("https://codeload.github.com/juj/emsdk/zip/master")
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute wget");

        Command::new("unzip")
            .arg(&emsdk_zip)
            .arg("-d")
            .arg(&siyo_dir)
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute unzip");
    }

    let mut emsdk = emsdk_dir.clone();
    emsdk.push("emsdk");

    Command::new(&emsdk)
        .arg("install")
        .arg("latest")
        .stdout(Stdio::null())
        .output()
        .expect("Failed to execute emsdk");

    Command::new(&emsdk)
        .arg("activate")
        .arg("latest")
        .stdout(Stdio::null())
        .output()
        .expect("Failed to execute emsdk");

    emsdk_dir
}

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
    // Create Siyo config folder if it doesn't exist.
    let data_dir = dirs::data_local_dir().unwrap();
    let mut siyo_dir = data_dir.clone();
    siyo_dir.push("cargo-siyo");
    fs::create_dir_all(&siyo_dir).unwrap();

    // Iterpret arguments.
    let arg = if let Some(arg) = std::env::args().skip(1).next() {
        arg
    } else {
        eprintln!("Usage:\n\n    \
            siyo dive     # Run in Dive environment\n    \
            siyo run      # Run natively\n    \
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
        "dive" => {
            eprintln!("TODO");
            std::process::exit(255);
        }
        "run" => {
            eprintln!("TODO");
            std::process::exit(255);
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

/*    

    println!("Configuring....");
    let emsdk_dir = get_emsdk(&siyo_dir);

    let mut emsdk_env = emsdk_dir.clone();
    emsdk_env.push("emsdk_env.sh");

    let name = get_name();

    println!("Building {}....", name);
    Command::new("sh")
        .arg("-c")
        .arg(format!("source {:?}; cargo build --release --target=wasm32-unknown-emscripten", &emsdk_env))
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute cargo");

    fs::create_dir_all("target/html").unwrap();

    Command::new("cp")
        .arg(format!("target/wasm32-unknown-emscripten/release/{}.js", name))
        .arg("target/html/")
        .output()
        .expect("failed to copy");

    Command::new("cp")
        .arg(format!("target/wasm32-unknown-emscripten/release/{}.wasm", name))
        .arg("target/html/")
        .output()
        .expect("failed to copy");

    */

//    println!("Built.");

/*    Command::new("xdg-open")
        .arg("http://[::1]:8000/")
        .output()
        .expect("failed to execute cargo");*/
    println!("Done.");
}
