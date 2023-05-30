 
use std::fmt::Error;
use std::fs::{File};
use std::io::{self, BufRead};


#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors= 3,
}

fn get_sign_enum_variant_right(value: &str) -> Option<Score> {
    match value {
        "X" => Some(Score::LOSE),
        "Y" => Some(Score::DRAW),
        "Z" => Some(Score::WIN),
        _ => None,
    }
}

fn get_sign_enum_variant(value: &str) -> Option<Sign> {
    match value {
        "A" => Some(Sign::Rock),
        "B" => Some(Sign::Paper),
        "C" => Some(Sign::Scissors),
        _ => None,
    }
}
#[derive(Copy, Clone)]
#[derive(PartialEq)]
enum Score {
    LOSE=0,
    DRAW=3,
    WIN=6,
}

fn get_score_enum_variant(value: &str,  _value2: &Option<Sign>) -> Result<i32,Error> {
    let score_value = match value {
        "win" => Score::WIN as i32,
        "draw" => Score::DRAW as i32,
        "lose" => Score::LOSE as i32,
        _ => 0,
    };

    let hand_value = match value {
        "win" =>{
            match _value2 {
                Some(Sign::Rock) => Sign::Paper as i32,
                Some(Sign::Scissors) => Sign::Rock as i32,
                Some(Sign::Paper) => Sign::Scissors as i32,
                None => 0,
            }
        },
        "draw" => {
            match _value2 {
            Some(Sign::Rock) => Sign::Rock as i32,
            Some(Sign::Scissors) => Sign::Scissors as i32,
            Some(Sign::Paper) => Sign::Paper as i32,
            None => 0,
        }},
        "lose" =>{
            match _value2 {
            Some(Sign::Rock) => Sign::Scissors as i32,
            Some(Sign::Scissors) => Sign::Paper as i32,
            Some(Sign::Paper) => Sign::Rock as i32,
            None => 0,
        }},
        _ => 0,
    };
    println!("{}", hand_value);

    let total =hand_value + score_value;

    Ok(total)
}

fn get_score(comp1: &str,comp2: &str) -> Result<i32, Error> {

    let mut total = 0;

    let  convCompGame = get_sign_enum_variant_right(comp2);
    let  convComp2 = get_sign_enum_variant(comp1);



   if convCompGame == Some(Score::WIN)  {
    if let Some(score) = get_score_enum_variant("win", &convComp2).ok() {
        total += score as i32;
    }
   } else if convCompGame == Some(Score::DRAW) {
    if let Some(score) = get_score_enum_variant("draw", &convComp2).ok() {
        total += score as i32;
    }
   } else if convCompGame == Some(Score::LOSE) {
    if let Some(score) = get_score_enum_variant("lose", &convComp2).ok() {
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
                let result = get_score( substrings[0],substrings[1]);
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