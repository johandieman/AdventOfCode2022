
use std::fmt::Debug;
use std::{fs::File, io};
use std::io::{BufRead};
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
                if line.contains("$") {
                    let args:Vec<&str> = line.split_whitespace().collect();
                    if args[1] == "cd" {
                        if args[2] != "/" && args[2] != ".." {


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
    
    let closest_sum = find_closest_sum(&tree, tree.root_id().unwrap(), 0, 0, 100000,0);
  
    println!("closest sum {:?}",closest_sum);

}

fn find_closest_sum(tree: &Tree<TreeNode>, node_id: NodeId, current_sum: i32, closest_sum: i32, target: i32, mut level:i32) -> (i32,i32) {
    let node = tree.get(node_id).unwrap();
    let new_sum = current_sum + node.data().value.parse::<i32>().unwrap_or(0); // for "dir" values

    println!("node  {:?}",node.data());
    
    if new_sum > target {
        println!("end big {:?}",closest_sum);        
        return (closest_sum,level);
    }
    println!("new  {:?}",new_sum);

    let new_closest_sum = if (target - new_sum).abs() < (target - closest_sum).abs() {
        new_sum
    } else {
        closest_sum
    };

    let is_all_empty =  node.children().all(|child| {
        let child_node = tree.get(child.node_id()).unwrap();
        child_node.data().value.is_empty()
    });

    if is_all_empty {

        return (new_closest_sum, level);
    }
 
    let mut updated_closest_sum = new_closest_sum;

    for child in node.children() {
        let child_sum = find_closest_sum(tree, child.node_id(), new_sum, new_closest_sum, target,level+1);

        println!("level  {:?}",child_sum.1);
        let new_child = updated_closest_sum+(child_sum.0* (level-child_sum.1).abs());
        updated_closest_sum = if (target - (new_child)).abs() < (target - updated_closest_sum).abs() {
            new_child
        } else {
            updated_closest_sum
        };
        println!("child updated {:?}",child_sum);
        println!("updated_closest_sum {:?}",updated_closest_sum);
    }


    level += 1;
    (updated_closest_sum, level)
}

fn pretty_print(tree: &Tree<TreeNode>, node_id: NodeId, depth: usize) {
    let node = tree.get(node_id).unwrap();
    let indent = "  ".repeat(depth);

    println!("{:?}{:?} ({:?}): {:?}", indent, node.data().id, node.data().value, node_id);

    for child in node.children() {
        pretty_print(tree, child.node_id(), depth + 1);
    }
}