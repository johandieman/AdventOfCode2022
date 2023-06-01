use std::collections::HashMap;
use std::{collections::HashSet, io};
use std::fs::{File};
use std::io::{BufRead};

pub mod sol2;
#[derive(PartialEq, Eq, Hash,Debug)]
struct HashTableS {
    key:char,
    value:i32,
}

pub fn main() {
   let numbs=  ('a'..='z').into_iter().collect::<Vec<char>>();
   let numbsbase2=  ('A'..='Z').into_iter().collect::<Vec<char>>();

    let mut hashScores = HashMap::new();

    for (i, c)in numbs.iter().enumerate(){
        let index = &i;
        hashScores.insert(*c,  *index as i32+1);
    }

    for (i, c)in numbsbase2.iter().enumerate(){
        let index = &i;
        hashScores.insert(*c, *index as i32+27);
    }


    let mut score = 0;
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let length = line.len();
                let mid = length/2;
                let (substring1,substring2) = line.trim().split_at(mid);

                let mut arr = HashSet::new();
                for i in substring1.chars().into_iter() {
                    if substring2.contains(i) {
                        arr.insert(i);
                    }
                }
                println!("{:?}",arr);

                for i in arr.iter() {
                    if let Some(item) = hashScores.get(i){
                        score += item;
                    }
                }
            }
        }
    } else {
        println!("Failed to open the file.");
    }

    println!("{}",score);

}