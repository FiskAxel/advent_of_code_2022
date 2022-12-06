fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Unreadable");
    
    for i in 0..input.len()-4 {
        if only_distinct_characters(&input[i..i+4]) {
            println!("Part 1: {}", i+4);
            break
        }
    }

    for i in 0..input.len()-14 {
        if only_distinct_characters(&input[i..i+14]) {
            println!("Part 2: {}", i+14);
            break
        }
    }
}

fn only_distinct_characters(input: &str) -> bool {
    let mut chars = input.clone();
    while chars.len() > 1 {
        let c: &str= &chars[0..1];
        chars = &chars[1..];
        if chars.contains(c) {
            return false;
        }
    }
    return true
}