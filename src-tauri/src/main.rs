
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[macro_use]
extern crate lazy_static;

// pub mod trie;
use std::{path::{Path, PathBuf}, process::{Command, exit}};


lazy_static! {
    static ref TRIE: trie::Trie = {
        let mut trie = trie::Trie::new();
        let path = std::env::var("PATH").unwrap();
        for p in path.split(":") {
            let path = Path::new(p);
            for entry in path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let name = path.file_name().unwrap().to_str().unwrap();
                    trie.insert(name);
                }
            }
        }
        trie
    };
}

mod trie {
    use std::{collections::HashMap, fs::File, io::Write};

    pub struct Trie {
        pub root: Node,
    }

    pub struct Node {
        pub children: HashMap<char, Node>,
        pub is_word: bool,
    }

    impl Trie {
        pub fn new() -> Self {
            Self {
                root: Node {
                    children: HashMap::new(),
                    is_word: false,
                },
            }
        }

        pub fn insert(&mut self, word: &str) {
            let mut current = &mut self.root;
            for c in word.chars() {
                current = current.children.entry(c).or_insert(Node {
                    children: HashMap::new(),
                    is_word: false,
                });
            }
            current.is_word = true;
        }

        pub fn contains(&self, word: &str) -> bool {
            let mut current = &self.root;
            for c in word.chars() {
                match current.children.get(&c) {
                    Some(node) => current = node,
                    None => return false,
                }
            }
            current.is_word
        }

        // pub fn complete(&self, word: &str, depth: usize) -> Vec<String> {
        //     let mut current = &self.root;
        //     let mut words = Vec::new();
        //     for c in word.chars() {
        //         match current.children.get(&c) {
        //             Some(node) => current = node,
        //             None => return words,
        //         }
        //     }
        //     self.complete_helper(current, word, depth, &mut words);
        //     words
        // }

        // pub fn complete_helper(&self, node: &Node, word: &str, depth: usize, words: &mut Vec<String>) {
        //     if depth == 0 {
        //         return;
        //     }
        //     for (c, node) in &node.children {
        //         let mut new_word = word.to_string();
        //         new_word.push(*c);
        //         if node.is_word {
        //             words.push(new_word);
        //         }
        //         self.complete_helper(node, &new_word, depth - 1, words);
        //     }
        // }

        // dump trie to dot file
            pub fn dump_dot(&self) {
                let mut file = File::create("trie.dot").unwrap();

                let mut dot = String::new();
                dot.push_str("digraph trie {\n");
                self.dump_dot_helper(&self.root, &mut dot);
                dot.push_str("}\n");

                file.write_all(dot.as_bytes()).unwrap();
            }

        pub fn dump_dot_helper(&self, node: &Node, dot: &mut String) {
            for (c, node) in &node.children {
                match node.children.len() {
                    0 => continue,
                    1 => (),
                    _ => {
                        let child = node.children.keys().next().unwrap();
                        dot.push_str(&format!("\"{}\" -> \"{}\";\n", c, child));
                    }
                }
                self.dump_dot_helper(node, dot);
            }
        }

        pub fn print_nodes(&self) {
            self.print_nodes_helper(&self.root, 0);
        }

        pub fn print_nodes_helper(&self, node: &Node, depth: usize) {
            for (c, node) in &node.children {
                println!("{}{}", " ".repeat(depth), c);
                self.print_nodes_helper(node, depth + 1);
            }
        }
    }
}

enum InputTypes {
    Command,
    File,
    Calculation
}

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

#[tauri::command]
fn autocomplete(input: &str) -> Vec<String> {
    todo!()
}


fn main() {

    // // build trie for binary names
    // let mut trie = trie::Trie::new();

    // // insert all binaries in path into trie
    // let path = std::env::var("PATH").unwrap();
    // for p in path.split(":") {
    //     let path = Path::new(p);
    //     for entry in path.read_dir().unwrap() {
    //         let entry = entry.unwrap();
    //         let path = entry.path();
    //         if path.is_file() {
    //             let name = path.file_name().unwrap().to_str().unwrap();
    //             trie.insert(name);
    //         }
    //     }
    // }

    // print trie
    // TRIE.dump_dot();
    // TRIE.print_nodes();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_input,
            kill,
            // autocomplete
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
