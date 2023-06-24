pub mod page;
pub mod string_ds;
pub mod tree;
use std::io;

use crate::page::Page;

use std::collections::HashMap;

fn add_page() {
    let mut file_name = String::new();
    println!("Add new Solution");
    println!("File Name?");
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let mut file_path = String::new();
    println!("Directory?");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");
    file_path = file_path.trim_end().to_string();
    let new_page = Page::new(file_name, file_path, "test".to_string());
    println!("{}", new_page.get_contents());
}

fn menu() {
    let mut menu: HashMap<String, fn()> = HashMap::new();
    menu.insert(String::from("add page"), add_page);
    loop {
        println!("Avail Commands");
        for key in menu.keys() {
            println!("{}", key);
        }
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if let Some(command) = menu.get(trimmed_input) {
                    command();
                } else {
                    println!("Invalid command.");
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn main() {}
