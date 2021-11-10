#[cfg(test)]
mod tests {
    #[test]
    fn test_alphabet_a() {
        let code_map = super::create(&"alphabet");
        let cd = &(code_map.unwrap());

        // A <-> 0
        assert_eq!(cd.convert_to_code(&'A'), &0);
        assert_eq!(cd.convert_to_char(&0), &'A');
    }
    #[test]
    fn test_alphabet_s() {
        let code_map = super::create(&"alphabet");
        let cd = &(code_map.unwrap());

        // S <-> 18
        assert_eq!(cd.convert_to_code(&'S'), &18);
        assert_eq!(cd.convert_to_char(&18), &'S');
    }
    #[test]
    fn test_alphabet_z() {
        let code_map = super::create(&"alphabet");
        let cd = &(code_map.unwrap());

        // Z <-> 25
        assert_eq!(cd.convert_to_code(&'Z'), &25);
        assert_eq!(cd.convert_to_char(&25), &'Z');
    }
}

use std::collections::HashMap;
use std::vec::Vec;

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
    pub fn is_char_target(&self, char: &char) -> bool {
        *(&self.map_char.contains_key(char))
    }
    pub fn size(&self) -> usize {
        *(&self.map_char.len())
    }
}

pub fn create(mode: &str) -> Result<CodeMap, &'static str> {
    let list_char_alphabet: Vec<char> = vec!(
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J','K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z'
    );
    
    let list_char: Option<Vec<char>> = match mode {
        "alphabet" => Some(list_char_alphabet),
        _ => None
    };

    if let Some(list_char) = list_char {
        let mut map_char: HashMap<char, i32> = HashMap::new();
        let mut map_char_rev: HashMap<i32, char> = HashMap::new();
    
        for (i, &char) in list_char.iter().enumerate() {
            let i_i32 = i as i32;
            map_char.insert(char, i_i32 + 1);
            map_char_rev.insert(i_i32 + 1, char);
        }
        Ok(CodeMap {
            map_char: map_char, 
            map_char_rev: map_char_rev
        })
    } else {
        Err("指定のモードが対象外です。")
    }
}
