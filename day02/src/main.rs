use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("src/input.txt")
        .expect("Unreadable");
    let ins: Vec<&str> =  input
        .trim()
        .lines()
        .collect();


    let rules1: HashMap<&str, i32> = HashMap::from([
        ("A X", 3 + 1), ("A Y", 6 + 2), ("A Z", 0 + 3),
        ("B X", 0 + 1), ("B Y", 3 + 2), ("B Z", 6 + 3),
        ("C X", 6 + 1), ("C Y", 0 + 2), ("C Z", 3 + 3),
    ]);
    let mut score = calculate_score(&ins, &rules1);
    println!("Part1: {}", score);


    let rules2: HashMap<&str, i32> = HashMap::from([
        ("A X", 3 + 0), ("A Y", 1 + 3), ("A Z", 2 + 6),
        ("B X", 1 + 0), ("B Y", 2 + 3), ("B Z", 3 + 6),
        ("C X", 2 + 0), ("C Y", 3 + 3), ("C Z", 1 + 6),
    ]);
    score = calculate_score(&ins, &rules2);
    println!("Part2: {}", score);

}

fn calculate_score(ins: &Vec<&str>, rules: &HashMap<&str, i32> ) -> i32 {
    let mut score: i32 = 0;
    for i in ins {
        score += rules[i];
    }
    return score;
}