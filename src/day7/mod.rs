use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::{fs::File, io};
use std::io::{BufRead};
use slab_tree::*;


struct TreeNode {
    id: String,
    value: String,
}

pub fn main(){

    let mut tree = TreeBuilder::new().with_root(TreeNode {id: String::from("/"),value: String::new() }).build();


    let mut flag: bool = false;

    let mut pointer:Option<NodeId> = None;

    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let root_id = tree.root_id().expect("root doesn't exist?");
                    let mut tree_entry= tree.get_mut(root_id).unwrap();
                
                

                    if line.contains("$") {
                        let args:Vec<&str> = line.split_whitespace().collect();

                        if args[1] == "cd" {

                            if args[2] != "/" {
                                if let Some(node_id) = pointer.as_mut() {

                                    *node_id = tree_entry.append(TreeNode {
                                        id: args[2].to_string(),
                                        value: String::new(),
                                    }).node_id();
                                }
                            }
                            flag = false;
                        } else if args[0] == "ls" {
                            flag = true;                            
                        }
                    } else {
                        if flag == true {
                            let args:Vec<&str> = line.split_whitespace().collect();


                            let mut val =tree.get_mut(pointer.unwrap()).unwrap();


                            match args[0].parse::<i32>() {
                                Ok(num) => {
                                    val.data().value = num.to_string();
                                }
                                Err(_) => {

                                }
                            }


                            
                        }
                    }





    }}}
}