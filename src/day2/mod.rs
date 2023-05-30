
use std::fmt::Error;
use std::fs::{File};
use std::io::{self, BufRead};
pub mod sol2;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors= 3,
}

fn get_sign_enum_variant(value: &str) -> Option<Sign> {
    match value {
        "X" => Some(Sign::Rock),
        "Y" => Some(Sign::Paper),
        "Z" => Some(Sign::Scissors),
        "A" => Some(Sign::Rock),
        "B" => Some(Sign::Paper),
        "C" => Some(Sign::Scissors),
        _ => None,
    }
}

enum Score {
    LOSE=0,
    DRAW=3,
    WIN=6,
}

fn get_score_enum_variant(value: &str,  _value2: &Option<Sign>) -> Result<i32,Error> {
    let mut total = 0;
    let score_value = match value {
        "win" => Score::WIN as i32,
        "draw" => Score::DRAW as i32,
        "lose" => Score::LOSE as i32,
        _ => 0,
    };


    if let Some(score) = _value2 {
        total += *score as i32 + score_value;
    }

    Ok(total)
}

fn get_score(comp1: &str, comp2: &str) -> Result<i32, Error> {

    let mut total = 0;

    let  convComp1 = get_sign_enum_variant(comp2);
    let  convComp2 = get_sign_enum_variant(comp1);

    if convComp1 == convComp2 {
        if let Some(score) = get_score_enum_variant("draw", &convComp1).ok() {
            total += score as i32;
        }
    }


   if convComp1 == Some(Sign::Rock) && convComp2 == Some(Sign::Scissors) {
    if let Some(score) = get_score_enum_variant("win", &convComp1).ok() {
        total += score as i32;
    }
   } else if convComp1 == Some(Sign::Rock) && convComp2 == Some(Sign::Paper) {
    if let Some(score) = get_score_enum_variant("lose", &convComp1).ok() {
        total += score as i32;
    }
   }

   if convComp1 == Some(Sign::Paper) && convComp2 == Some(Sign::Rock) {
    if let Some(score) = get_score_enum_variant("win", &convComp1).ok() {
        total += score as i32;
    }
   } else if convComp1 == Some(Sign::Paper) && convComp2 == Some(Sign::Scissors) {
    if let Some(score) = get_score_enum_variant("lose", &convComp1).ok() {
        total += score as i32;
    }
   }

   if convComp1 == Some(Sign::Scissors) && convComp2 == Some(Sign::Paper) {
    if let Some(score) = get_score_enum_variant("win", &convComp1).ok() {
        total += score as i32;
    }
   } else if convComp1 == Some(Sign::Scissors) && convComp2 == Some(Sign::Rock) {
    if let Some(score) = get_score_enum_variant("lose", &convComp1).ok() {
        total += score as i32;
    }
   }


   Ok(total)

}
pub fn main() {
    let mut arr = vec![];
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {

                let substrings: Vec<&str> = line.split(' ').collect();
                let result = get_score(substrings[0], substrings[1]);
                if let Some(result) = result.ok() {
                    arr.push(result);
                }
            }
        }
    } else {
        println!("Failed to open the file.");
    }


    let mut  sum = 0;

    for i in arr.iter(){
        sum += *i
    }

    println!("{}", sum);

    




}