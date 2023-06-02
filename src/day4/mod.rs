
use std::fs::{File};
use std::io::{BufRead};
use std::{collections::HashSet, io};
pub mod sol2;

pub fn main(){
    let mut score: i32 = 0;
    if let Ok(file) = File::open("input.txt"){
        let reader = io::BufReader::new(file);
        for line in reader.lines(){
            if let Ok(line) = line{
                let ranges:Vec<&str> = line.split(',').collect();

                let min_max1:Vec<&str> = ranges[0].split('-').collect();
                let min_max2:Vec<&str> = ranges[1].split('-').collect();

                let range1: HashSet<i32> = (min_max1[0].parse().unwrap()..=min_max1[1].parse().unwrap()).collect();
                let range2: HashSet<i32> = (min_max2[0].parse().unwrap()..=min_max2[1].parse().unwrap()).collect();

                if range1.is_subset(&range2) || range2.is_subset(&range1) {
                    score +=1;
                }

                
            }
        }
    }
    println!("{}",score);
}