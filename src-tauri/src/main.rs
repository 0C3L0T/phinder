
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{path::{Path, PathBuf}, process::{Command, exit}};

enum InputTypes {
    Command,
    File,
    Calculation
}

// TODO build trie for binaries

fn execute_binary(binary_path: &Path) -> Result<(), String> {
    Command::new(binary_path)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// in_path returns a boolean if the binary is in the path
fn in_path(binary: &str) -> Result<Option<PathBuf>, std::env::VarError> {
    let path = std::env::var("PATH")?;
    for p in path.split(":") {
        let path = Path::new(p).join(binary);
        if path.exists() {
        return Ok(Some(path));
        }
    }
    Ok(None)
}

#[tauri::command]
fn kill() {
    exit(0);
}

#[tauri::command]
fn handle_input(input: &str) {
    println!("You entered: {}", input);
    match in_path(input) {
        Ok(Some(path)) => {
            println!("Found binary at: {}", path.display());
            match execute_binary(&path) {
                Ok(_) => {
                    println!("Successfully executed binary");
                    exit(0);
                },
                Err(e) => println!("Error executing binary: {}", e),
            }
        },
        Ok(None) => println!("Could not find binary"),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_input,
            kill
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
