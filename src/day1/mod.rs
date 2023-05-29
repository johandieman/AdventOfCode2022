use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct Dictionary<K> {
    data: HashMap<K, i32>,
}

impl<K> Dictionary<K>

where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        Dictionary {
            data: HashMap::new(),
        }
    }



    fn insert_add(&mut self, key: K, value: i32) {
        if let Some(current_value) = self.data.get_mut(&key) {
            *current_value += value;
        } else {
            self.data.insert(key, value);
        }
    }

    

}

pub fn main() {

    let mut dictionary = Dictionary::new();
    let mut current_key = 1;

    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.trim().is_empty() {
                    current_key += 1;
                } else {
                    if let Ok(number) = line.parse::<i32>() {
                        dictionary.insert_add(current_key,number );
                    }
                }
            }
        }
    } else {
        println!("Failed to open the file.");
    }

    if let Some((max_key, _)) = dictionary.data.iter().max_by_key(|(_, v)| *v){
        println!("{}", max_key);
    } else {
        println!("No max found");
    }
}
