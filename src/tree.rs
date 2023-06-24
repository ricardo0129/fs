// Implementation of a tree data structure for storing pages
// Exactly like Trie, but instead of characters we use a hashmap to map attibutes to children

use crate::page::Page;
use std::collections::HashMap;

struct Node {
    child: Vec<Node>,
    names: HashMap<i32, usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            child: vec![],
            names: HashMap::new(),
        }
    }

    fn insert(&mut self, p: Page) {
        let root = self;
    }
}

struct Tree {
    attributes: HashMap<String, i32>,
}

impl Tree {
    fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    fn attr_index(&mut self, attr: &String) -> i32 {
        if !self.attributes.contains_key(attr) {
            self.attributes
                .insert(attr.clone(), self.attributes.len() as i32);
        }
        *self.attributes.get(attr).unwrap()
    }
}
