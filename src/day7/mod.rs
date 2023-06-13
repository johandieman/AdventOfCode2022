
use std::fmt::Debug;
use std::{fs::File, io};
use std::io::{BufRead};
use itertools::Itertools;
use slab_tree::*;

#[derive(Debug)]
struct TreeNode {
    id: String,
    value: String,
}

pub fn main(){

    let mut tree = TreeBuilder::new().with_root(TreeNode {id: String::from("/"),value: String::new() }).build();


    let mut flag: bool = false;

    
    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        let root_id = tree.root_id().expect("root doesn't exist?");
        let mut pointer:Option<NodeId> = Some(root_id);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{:?}",pointer);
                if line.contains("$") {
                    let args:Vec<&str> = line.split_whitespace().collect();
                    println!("{:?}",args);

                    if args[1] == "cd" {
                        if args[2] != "/" && args[2] != ".." {
                            println!("{:?}","updateing");

                            // if let Some(mut node) = tree.get_mut(pointer.unwrap()) {
                            let current_pointer = pointer.unwrap();
                            let mut current_node = tree.get_mut(current_pointer).unwrap();
                            // let child_ids: Vec<NodeId> = current_node.;

                            let children = current_node.as_ref();

                            let mut contained = false;
                            for child in children.children() {
                                if child.data().id == args[2] {
                                    pointer = Some(child.node_id());
                                    contained=true;
                                    break;
                                }
                            }

                            if contained == false {
                                let new_child=  current_node.append(TreeNode {
                                    id: args[2].to_string(),
                                    value: String::new(),
                                }).node_id();

                                pointer = Some(new_child);
                            }
                            
                        } else if args[2] == ".."{
                            let current_pointer = pointer.unwrap();
                            pointer= Some(tree.get(current_pointer).unwrap().parent().unwrap().node_id());
                        }
                        flag = false;
                    }  else if args[1] == "ls" {
                        flag = true;                            
                    }
                } else {
                    if flag == true {
                        let args:Vec<&str> = line.split_whitespace().collect();
                        if let Some(node_id) = pointer {
                            let mut val = tree.get_mut(node_id).unwrap();
                        
                            val.append(TreeNode {
                                id: args[1].to_string(),
                                value: args[0].to_string(),
                            });                            
                        }

                        

                    }
                }

            }
        }
    }

    pretty_print(&tree, tree.root_id().unwrap(), 0);
    
    let closest_sum = find_closest_sum(&tree, tree.root_id().unwrap(), 0, 0, 100000);

    

    println!("{:?}",closest_sum);

}

fn find_closest_sum(tree: &Tree<TreeNode>, node_id: NodeId, current_sum: i32, closest_sum: i32, target: i32) -> i32 {
    let node = tree.get(node_id).unwrap();
    let new_sum = current_sum + node.data().value.parse::<i32>().unwrap_or(0); // for "dir" values

    println!("node{:?}",node.data());
    println!("new{:?}",new_sum);
    // println!("closest{:?}",closest_sum);
    if new_sum > target {
        return closest_sum;
    }

    let new_closest_sum = if (target - new_sum).abs() < (target - closest_sum).abs() {
        new_sum
    } else {
        closest_sum
    };

    let mut children = node.children();



    let is_all_empty = children.all(|child| {
        let child_node = tree.get(child.node_id()).unwrap();
        child_node.data().value.is_empty()
    });

    if is_all_empty{
        return new_closest_sum;
    }
 
    let mut updated_closest_sum = new_closest_sum;

    for child in children {
        println!("child{:?}",child.data());
        let child_sum = find_closest_sum(tree, child.node_id(), new_sum, updated_closest_sum, target);
        updated_closest_sum = if (target - child_sum).abs() < (target - updated_closest_sum).abs() {
            child_sum
        } else {
            updated_closest_sum
        };
    }

    updated_closest_sum
}


fn pretty_print(tree: &Tree<TreeNode>, node_id: NodeId, depth: usize) {
    let node = tree.get(node_id).unwrap();
    let indent = "  ".repeat(depth);

    println!("{:?}{:?} ({:?}): {:?}", indent, node.data().id, node.data().value, node_id);

    for child in node.children() {
        pretty_print(tree, child.node_id(), depth + 1);
    }
}