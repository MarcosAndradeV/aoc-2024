use std::{fs, str::FromStr};

use regex;

const INPUT_PATH: &str = "tests/day_03/input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("no file error");
    part_1(&input);
    part_2(&input);
}

fn part_2(input: &String) {
    let re = regex::Regex::new(r#"(mul\(\d{1,3}\,\d{1,3}\))|(do(n't)?\(\))"#).expect("regex error");
    let mut mats = re.find_iter(input);
    let mut result = 0;
    let mut a = true;
    while let Some(mat) = mats.next() {
        match mat.as_str() {
            "do()" => {
                a = true;
            }
            "don't()" => {
                a = false;
            }
            mul if a => {
                result += mul.parse::<CustomMul>().expect("no match found").eval();
            }
            _ => {},
        }
    }
    println!("Result: {result}")
}

fn part_1(input: &String) {
    let re = regex::Regex::new(r#"mul\(\d{1,3}\,\d{1,3}\)"#).expect("regex error");
    let mut mats = re.find_iter(input);
    let mut result = 0;
    while let Some(mat) = mats.next() {
        result += mat
            .as_str()
            .parse::<CustomMul>()
            .expect("no match found")
            .eval();
    }
    println!("Result: {result}")
}

struct CustomMul {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ParseCustomMul;

impl FromStr for CustomMul {
    type Err = ParseCustomMul;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix("mul(")
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseCustomMul)?;
        Ok(CustomMul {
            x: x.parse().map_err(|_| ParseCustomMul)?,
            y: y.parse().map_err(|_| ParseCustomMul)?,
        })
    }
}

impl CustomMul {
    pub fn eval(self) -> i32 {
        self.x * self.y
    }
}
