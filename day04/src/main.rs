use regex::Regex;

fn main() {
    let mut contains: u16 = 0;
    let mut overlaps: u16 = 0;
    for p in get_parsed_input() {
        let lo1 = p[0];
        let hi1 = p[1];
        let lo2 = p[2];
        let hi2 = p[3];
        if lo1 <= lo2 && hi2 <= hi1 || 
           lo1 >= lo2 && hi2 >= hi1 {
            contains += 1;
            overlaps += 1;
        }
        else if lo1 <= lo2 && hi2 <= lo1 ||
                lo1 <= hi2 && lo2 <= lo1 ||
                hi1 <= lo2 && hi2 <= hi1 || 
                hi1 <= hi2 && lo2 <= hi1 {
            overlaps += 1;
        }
    }
    println!("Part 1: {}", contains);
    println!("Part 2: {}", overlaps);
}


fn get_parsed_input() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("src/input.txt").expect("Unreadable");
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap(); 
    let mut pairs: Vec<Vec<i32>> = vec![];
    for i in input.trim().lines() {
        for caps in re.captures_iter(i) {
            pairs.push(vec![
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<i32>().unwrap(),
                caps[3].parse::<i32>().unwrap(),
                caps[4].parse::<i32>().unwrap()
            ]);
        }
    }
    return pairs;
}