use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::{io};
use std::fs::{File};
use std::io::{BufRead};


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


    let mut lines:HashMap<i32, Vec<String>> = HashMap::new();
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        let mut counter = 1;
        let mut k = 1;
        for line in reader.lines() {
            if let Ok(line) = line {
                if counter % 3 == 0 {
                    if let Some(item) = lines.get(&k) {
                        let mut updated_item = item.to_vec();
                        updated_item.push(line);
                        lines.insert(k, updated_item);
                    }
                    k +=1;
                } else{   


                    if let Some(item) = lines.get(&k) {

                        let mut updated_item = item.to_vec();

                        updated_item.push(line);


                        lines.insert(k, updated_item);
                    } else {


                        let mut new_item = Vec::new();
                        new_item.push(line);
                        lines.insert(k, new_item);
                    }
                }
                counter+=1;
            }
        }
    } else {
        println!("Failed to open the file.");
    }
    let mut score = 0;


    let mut arr = HashSet::new();
    let mut mem:HashSet<char> = HashSet::new();
    for (_,j) in lines.iter() {
        arr.clear();
        mem.clear();
        for (index,item) in j.iter().enumerate(){
            if j.len() -1 == index {
                break;
            }

            if j.len() -1 == index+1 {
                println!("char{:?}",mem);

                for i in mem.iter() {

                    if j[index+1].contains(&i.to_string()) {
                        arr.insert(*i);
                    }
                }
            }else if index != j.len()-1{
                for i in item.chars(){
                
                    if j[index+1].contains(i){
                        mem.insert(i);
                    }
                }
            } 
            println!("arr{:?}",arr);
            println!("mem{:?}",mem);
        }


        for i in arr.iter() {
            if let Some(item) = hashScores.get(i){
                score += item;
            }
        }
    }

    println!("{}",score);
    


}