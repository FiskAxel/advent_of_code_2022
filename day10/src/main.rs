fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    
    println!("Part 2:");
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;
    for line in lines {
        cycle += 1;
        crt_print(cycle, x);
        signal_strength += update_signal_strength(x, cycle);
        
        match line {
            "noop" => { continue },
            _ => {
                cycle += 1;
                crt_print(cycle, x);
                signal_strength += update_signal_strength(x, cycle);

                let split: Vec<&str> = line.split(" ").collect();
                let dx: i32 = split[1].parse().unwrap();
                x += dx;
            }
        }
    }
    println!("Part 1: {}", signal_strength);
    
}

fn update_signal_strength(x: i32, cycle: i32) -> i32 {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        return cycle * x;
    }
    return 0;
}

fn crt_print(cycle: i32, x: i32) {
    let crt = cycle % 40 - 1;
    if crt == x-1 || crt == x || crt == x+1 {
        print!("#")
    } else {
        print!(".");
    } 
    if cycle % 40 == 0 { 
        println!(); 
    }
}