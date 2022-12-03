fn main() {
    let input = std::fs::read_to_string("src/input.txt")
        .expect("Unreadable");
    let rucksacks: Vec<&str> = input
        .trim()
        .lines()
        .collect();
    
    let mut sum: u32 = 0;
    for sack in &rucksacks {
        let half = sack.len() / 2;
        let compartment1 = &sack[..half];
        let compartment2 = &sack[half..];
        for c in compartment1.chars() {
            if compartment2.contains(c) {
                sum += get_priority(c);
                break;
            }
        }
    }
    println!("Part 1: {}", sum);

    sum = 0;
    let mut i = 0;
    while i < rucksacks.len() {
        let elf1 = rucksacks[i];
        let elf2 = rucksacks[i + 1];
        let elf3 = rucksacks[i + 2];
        for c in elf1.chars() {
            if elf2.contains(c) && elf3.contains(c) {
                sum += get_priority(c);
                break;
            }
        }
        i += 3;
    }
    println!("Part 2: {}", sum);
}

fn get_priority(c: char) -> u32 {
    (c as u32 - 38) % 58
}
