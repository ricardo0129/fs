use std::fs::File;
use std::io::Read;
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
pub struct Page {
    file_name: String,
    path: String,
    description: String,
    attributes: Vec<String>,
}

impl Page {
    pub fn new(file_name: String, path: String, description: String) -> Self {
        Self {
            file_name: file_name,
            path: path,
            description: description,
            attributes: vec![],
        }
    }

    pub fn contains_category(&self, category: &String) -> bool {
        self.attributes.contains(&category)
    }

    pub fn add_category(&mut self, category: String) {
        if !self.contains_category(&category) {
            self.attributes.push(category);
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_contents(&self) -> String {
        if let Some(contents) = read_file(&self.path) {
            return contents;
        } else {
            eprintln!("File is missing");
            return String::new();
        }
    }
}
