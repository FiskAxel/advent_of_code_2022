use regex::Regex;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Unreadable");
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut crates: Vec<&str> = input[0].lines().collect();
    crates.reverse();
    let instructions: Vec<&str> = input[1].lines().collect();

    let num_of_stacks = (&crates[0].len() + 1) / 4;
    let mut crate_stacks: Vec<Vec<char>> = vec![vec![]; num_of_stacks];
    for i in 1..crates.len() {
        for j in 0..num_of_stacks {
            let c: char = crates[i].as_bytes()[j*4 + 1] as char;
            if c != ' ' {
                crate_stacks[j].push(c);
            }
        }
    }
    let mut crate_stacks2 = crate_stacks.clone();


    let re = Regex::new(r"\D+ (\d+) \D+ (\d+) \D+ (\d+)").unwrap();
    for i in &instructions {
        let caps = re.captures(i).unwrap();
        let move_num: u8 = caps[1].parse::<u8>().unwrap();
        let from: usize = caps[2].parse::<usize>().unwrap() - 1;
        let to: usize = caps[3].parse::<usize>().unwrap() - 1;

        for _ in 0..move_num {
            let cratey = crate_stacks[from].pop().unwrap();
            crate_stacks[to].push(cratey)
        }
    }
    println!("Part 1: {}", get_top_crates(&crate_stacks));


    for i in &instructions {
        let caps = re.captures(i).unwrap();
        let move_num: usize = caps[1].parse::<usize>().unwrap();
        let from: usize = caps[2].parse::<usize>().unwrap() - 1;
        let to: usize = caps[3].parse::<usize>().unwrap() - 1;

        let mut stack: Vec<char> = vec![];
        for _ in 0..move_num {
            let cratey = crate_stacks2[from].pop().unwrap();
            stack.push(cratey);
        }
        stack.reverse();
        crate_stacks2[to].append(&mut stack);
    }
    print!("Part 2: {}", get_top_crates(&crate_stacks2));
}

fn get_top_crates(crate_stacks: &Vec<Vec<char>>) -> String {
    let mut top_crates = String::new();
    for i in crate_stacks {
        top_crates.push(i[i.len() - 1]);
    }
    return top_crates;
}
