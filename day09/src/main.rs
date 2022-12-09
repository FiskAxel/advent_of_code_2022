fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    println!("Part 1: {}", pull_rope(2, &lines));
    println!("Part 2: {}", pull_rope(10, &lines));
}

fn pull_rope(len: i32, lines: &Vec<&str>) -> usize {
    let mut rope = vec![];
    for _ in 0..len {
        rope.push(Pos { x: 0, y: 0 })
    }
    let mut tail_positions: Vec<Pos> = vec![Pos { x: 0, y: 0 }];
    for line in lines {
        let dir: Vec<&str> = line.split(" ").collect();
        let steps: i32 = dir[1].parse().unwrap();
        for _ in 0.. steps{
            match dir[0] {
                "U" => {
                    rope[0].y += 1;
                }, "D" => {
                    rope[0].y -= 1;
                }, "L" => {
                    rope[0].x -= 1;
                }, "R" => {
                    rope[0].x += 1;
                },_ => {}
            }
            for i in 1..len as usize {
                if move_tail(&rope[i-1], &rope[i]) {
                    let dx = get_delta(rope[i-1].x, rope[i].x);
                    let dy = get_delta(rope[i-1].y, rope[i].y);
                    rope[i] = Pos { x: rope[i].x + dx, y: rope[i].y + dy};

                    if i as i32 == len-1 && !tail_positions.contains(&rope[i]) {
                        tail_positions.push(Pos { x: rope[i].x, y: rope[i].y});
                    }
                }
            }       
        }
    }
    return tail_positions.len();
}

fn move_tail(head: &Pos, tail: &Pos) -> bool{
    if 1 < (head.x - tail.x).abs() ||
       1 < (head.y - tail.y).abs()  {
            return true;
    }
    return false;
}

fn get_delta(a: i32, b: i32) -> i32 {
    match a - b {
        1.. => 1,
        0 => 0,
        _ => -1
    }
}


struct Pos {
    x: i32,
    y: i32
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}
