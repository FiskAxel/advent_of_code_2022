fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut trees: Vec<Vec<u32>> = vec![];
    const RADIX: u32 = 10;
    for line in input.lines() {
        let row: Vec<u32> = line
            .chars()
            .map(|n| n.to_digit(RADIX).unwrap())
            .collect();
        trees.push(row)
    }

    let mut visible_trees: Vec<Vec<u32>> = vec![];
    // DOWN
    for x in 0..trees.len() {
        let mut highest: i32 = -1;
        for y in 0..trees.len() {
            highest = inner(trees[y][x].clone(), &mut visible_trees, highest, x, y);
            if highest == 9 {
                break;
            }
        }
    }
    // UP
    for x in 0..trees.len() {
        let mut highest: i32 = -1;
        for y in (0..trees.len()).rev() {
            highest = inner(trees[y][x].clone(), &mut visible_trees, highest, x, y);
            if highest == 9 {
                break;
            }
        }
    }
    // RIGHT
    for y in 0..trees.len() {
        let mut highest: i32 = -1;
        for x in 0..trees.len() {
            highest = inner(trees[y][x].clone(), &mut visible_trees, highest, x, y);
            if highest == 9 {
                break;
            }
        }
    }
    // LEFT
    for y in 0..trees.len() {
        let mut highest: i32 = -1;
        for x in (0..trees.len()).rev() {
            highest = inner(trees[y][x].clone(), &mut visible_trees, highest, x, y);
            if highest == 9 {
                break;
            }
        }
    }
    println!("Part 1: {}", visible_trees.len());

    let mut best_scenic_score = 0;
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            let score = calculate_scenic_score(x, y, trees.clone());
            if best_scenic_score < score {
                best_scenic_score = score;
            }
        }
    }
    println!("Part 2: {}", best_scenic_score); // Slow

}

// To get rid of some repeated code
fn inner(tree: u32, visible_trees: &mut Vec<Vec<u32>>,  mut highest: i32, x: usize, y: usize) -> i32 {
    if highest < tree as i32 {
        highest = tree as i32;
        let pos = vec![x as u32, y as u32];
        if !visible_trees.contains(&pos) {
            visible_trees.push(pos.clone());
        }
    }
    return highest;
}

fn calculate_scenic_score(x: usize, y: usize, trees: Vec<Vec<u32>>) -> u32 {
    let tree = trees[y][x];
    
    let mut num_down: u32 = 0;
    for i in y+1..trees.len() {
        num_down += 1;
        if tree <= trees[i][x] {
            break;
        } 
    }
    let mut num_up: u32 = 0;
    for i in (0..y).rev() {
        num_up += 1;
        if tree <= trees[i][x] {
            break;
        } 
    }
    let mut num_right: u32 = 0;
    for i in x+1..trees.len() {
        num_right += 1;
        if tree <= trees[y][i] {
            break;
        } 
    }
    let mut num_left: u32 = 0;
    for i in (0..x).rev() {
        num_left += 1;
        if tree <= trees[y][i] {
            break;
        } 
    }

    return num_down * num_up * num_right * num_left;
}