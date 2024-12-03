use std::fs;

const INPUT_PATH: &str = "tests/day_02/input.txt";
const EXAMPLE_PATH: &str = "tests/day_02/example.txt";

fn main() {
    let input: Vec<Vec<i32>> = fs::read_to_string(INPUT_PATH)
        .expect("no file error")
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().expect("no number error"))
                .collect()
        })
        .collect();
    part_1(&input);
    part_2(&input);
    let input: Vec<Vec<i32>> = fs::read_to_string(EXAMPLE_PATH)
        .expect("no file error")
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().expect("no number error"))
                .collect()
        })
        .collect();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<Vec<i32>>) {
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

fn part_2(input: &Vec<Vec<i32>>) {
    let mut safe_count = 0;
    'report: for report in input {
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            let diffs = new_report.windows(2).map(|elem| {
                if elem.len() == 2 {
                    elem[0] - elem[1]
                } else {
                    elem[0]
                }
            });
            let all_inc_or_dec = diffs.clone().all(|e| e > 0) || diffs.clone().all(|e| e < 0);
            let differ_by = diffs.clone().all(|e| {
                let diff = e.abs();
                1 <= diff && diff <= 3
            });
            if all_inc_or_dec && differ_by {
                safe_count += 1;
                continue 'report;
            }
        }
    }

    println!("Part 2 - Result: {}", safe_count);
}
