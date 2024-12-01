use std::{fs, iter::zip};

const INPUT_PATH: &str = "tests/day_01/input.txt";

fn main(){
    let source = fs::read_to_string(INPUT_PATH).expect("file input error");
    let lines: Vec<&str> = source.lines().collect();
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    for line in lines {
        let l: Vec<&str> = line.split("   ").collect();
        list1.push(l[0].parse::<i64>().expect("No number found"));
        list2.push(l[1].parse::<i64>().expect("No number found"));
    }
    list1.sort();
    list2.sort();
    part_1(&list1, &list2);
    part_2(&list1, &list2);

}

fn part_2(list1: &Vec<i64>, list2: &Vec<i64>) {
    let mut similarity_score: i64 = 0;
    for n1 in list1 {
        let mut ntimes = 0;
        for n2 in list2 {
            if n1 == n2 {
                ntimes+=1;
            }
        }
        similarity_score += n1 * ntimes;
    }
    println!("Result {}", similarity_score);
}

fn part_1(list1: &Vec<i64>, list2: &Vec<i64>) {
    let mut distace: i64 = 0;
    for (n1, n2) in zip(list1, list2) {
        distace += (n1 - n2).abs();
    }

    println!("Result {}", distace);
}
