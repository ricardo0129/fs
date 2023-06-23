struct Page {
    file_name: String,
    path: String,
    description: String,
    attributes: Vec<String>,
}

impl Page {
    fn new(file_name: String, path: String, description: String) -> Self {
        Self {
            file_name: file_name,
            path: path,
            description: description,
            attributes: vec![],
        }
    }

    fn contains_category(&self, category: &String) -> bool {
        self.attributes.contains(&category)
    }

    fn add_category(&mut self, category: String) {
        if !self.contains_category(&category) {
            self.attributes.push(category);
        }
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_contents(&self) -> String {
        if let Some(contents) = read_file(&self.path) {
            return contents;
        } else {
            eprintln!("File is missing");
            return String::new();
        }
    }
}
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn read_file(path: &String) -> Option<String> {
    let p = "src/waif.cpp";
    println!("{}-{}", p, path);
    println!("{}", path);
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Some(contents),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

fn main() {
    let pathbuff = get_current_working_dir().unwrap();
    //println!("{}", pathbuff.to_str().unwrap());
    //let v = read_file(&"src/waif.cpp".to_string()).unwrap();
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
    let new_page = Page::new(file_name, file_path, "test".to_string());
    println!("{}", new_page.get_contents());
}
