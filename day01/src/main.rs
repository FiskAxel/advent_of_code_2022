fn main() {
    let elf_inventories = get_parsed_input();

    let mut calorie_counts: Vec<i32> = vec![];
    for elf in elf_inventories {
        calorie_counts.push(elf.iter().sum());
    }
    calorie_counts.sort();
    calorie_counts.reverse();

    println!("Part 1: {}", calorie_counts[0]);
    println!("part 2: {}", calorie_counts[0..3].iter().sum::<i32>());
}


fn get_parsed_input() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("src/input.txt").expect("Unreadable");
    
    let raw_inventories: Vec<&str> = input
        .trim()
        .split("\r\n\r\n")
        .collect();

    let mut parsed_inventories: Vec<Vec<i32>> = vec![vec![]];
    for inventory in raw_inventories {
        parsed_inventories.push(inventory
            .lines()
            .map(|n| n.parse::<i32>().expect("parse error"))
            .collect()
        );
    }

    return parsed_inventories;
}