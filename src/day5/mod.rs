use std::collections::HashMap;
use std::{fs::File, io};
use std::io::{BufRead};
use itertools::Itertools;


pub fn main(){


    let mut has_table:HashMap<usize, Vec<char>> = HashMap::new();


    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("[") || line.contains("]") {

                    let mut div = 0;
                    for (i,c) in line.chars().enumerate(){

                        if i % 4 == 0 {
                            let charsLine:Vec<char> = line.chars().collect();
                            if c == '[' {


                                if let Some(item) = has_table.get(&div) {

                                    let mut updated_item = item.to_vec();
            
                                    updated_item.push(charsLine[i+1]);
            
                                    *has_table.get_mut(&div).unwrap() = updated_item;
                                } else {
                                    let mut new_item = Vec::new();
                                    new_item.push(charsLine[i+1]);
                                    has_table.insert(div, new_item);
                                }

                            }
                            div +=1;
                        }
                    }
                } else if line.contains("move") {
                    let items:Vec<&str> = line.split_whitespace().collect();

                    let mut action = vec![];
                    for (k,i) in items.iter().enumerate(){

                        match i.parse::<i32>(){
                            Ok(n) => {
                                if k > 2 {

                                    action.push(n as usize - 1);  
                                } else {
                                    action.push(n as usize);  

                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        }

                    }
                    println!("{:?}", has_table);

                    if let Some(item) = has_table.get(&action[1]) {
                        let cloned_item =item.clone();
                        let (left, right) = cloned_item.split_at(action[0]);
                        println!("left{:?}", left);
                        println!("right{:?}", right);
                        
                        *has_table.get_mut(&action[1]).unwrap() = right.to_vec();

                        if let Some(subitem) =has_table.get(&action[2]) {
                            let copied = Vec::from(left.to_vec());


                            //! SOLUTION1 copied.reverse();

                            let concated = [copied,subitem.clone()].concat();
                            *has_table.get_mut(&action[2]).unwrap() = concated;
                        }
                    }

                    println!("{:?}", has_table);


                } else {
                    continue;
                }
            }
        }
    }


    for (_,i) in has_table.iter().sorted() {
        if let Some(line) = i.first() {

            print!("{:?}", line);
        }
        // let fi = i.pop();
    }

    // println!("{:?}", has_table);
}

LLWJRBHVZ