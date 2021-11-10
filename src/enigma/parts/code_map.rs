use std::collections::HashMap;

pub struct CodeMap {
    map_char: HashMap<char, i32>,
    map_char_rev: HashMap<i32, char>
}

impl CodeMap {
    pub fn convert_to_code(&self, char: &char) -> &i32 {
        &self.map_char[char]
    }
    pub fn convert_to_char(&self, code: &i32) -> &char {
        &self.map_char_rev[code]
    }
}

pub fn create(list_char: std::vec::Vec<char>) -> CodeMap {
    let mut map_char: HashMap<char, i32> = HashMap::new();
    let mut map_char_rev: HashMap<i32, char> = HashMap::new();

    for (i, &char) in list_char.iter().enumerate() {
        let i_i32 = i as i32;
        map_char.insert(char, i_i32 + 1);
        map_char_rev.insert(i_i32 + 1, char);
    }
    CodeMap {
        map_char: map_char, 
        map_char_rev: map_char_rev
    }
}