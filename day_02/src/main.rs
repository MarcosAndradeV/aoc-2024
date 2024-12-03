use std::fs;

const INPUT_PATH: &str = "tests/day_02/input.txt";
const EXAMPLE_PATH: &str = "tests/day_02/example.txt";

fn main(){
    // part_1();
    part_2();
}

fn part_1() {
    let input: Vec<Vec<i32>> = fs::read_to_string(INPUT_PATH)
        .expect("no file error")
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().expect("no number error"))
                .collect()
        })
        .collect();
    let mut safe_count = 0;
    for report in input {
        let diffs = report.windows(2).map(|elem| {
            if elem.len() == 2 {
                elem[0] - elem[1]
            } else {
                elem[0]
            }
        });
        //.collect::<Vec<i32>>();
        let all_inc_or_dec = diffs.clone().all(|e| e > 0) || diffs.clone().all(|e| e < 0);
        let differ_by = diffs.clone().all(|e| {
            let diff = e.abs();
            1 <= diff && diff <= 3
        });
        if all_inc_or_dec && differ_by {
            safe_count += 1;
        }
    }

    println!("Part 1 - Result: {}", safe_count);
}

fn part_2() {
    let input: Vec<Vec<i32>> = fs::read_to_string(EXAMPLE_PATH)
        .expect("no file error")
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().expect("no number error"))
                .collect()
        })
        .collect();
    let safe_count = 0;
    for report in input {
        dbg!(report);
    }

    println!("Part 2 - Result: {}", safe_count);
}
