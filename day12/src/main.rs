use std::collections::HashMap;
fn main() {
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    let mut map: Vec<Vec<char>> = get_map(input);
    let start: Pos = get_pos(&'S', &map);
    let goal: Pos = get_pos(&'E', &map);
    let a_positions = get_every_a_position(&map);
    map[start.y as usize][start.x as usize] = 'a';
    map[goal.y as usize][goal.x as usize] = 'z';

    let mut result: i32 = a_star(start, &goal, &map);
    println!("Part 1: {}", result);

    for pos in a_positions {
        let steps: i32 = a_star(pos, &goal, &map);
        if steps < result {
            result = steps;
        } 
    }
    println!("Part 2: {}", result); // runs in about 20 seconds.
}

fn get_map(input: String) -> Vec<Vec<char>>{
    let mut map: Vec<Vec<char>> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    return map;
}

fn get_pos(c: &char, map: &Vec<Vec<char>>) -> Pos {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == *c {
                x = j as i32;
                y = i as i32;
            }
        } 
    }

    let mut z = map[y as usize][x as usize];
    if z == 'S' {
        z = 'a';
    }else if z == 'E' {
        z = 'z';
    }
    
    Pos { x: x, y: y, z: z, steps: 0, distance: 1 }
}

fn get_every_a_position(map: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut every_a: Vec<Pos> = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'a' && b_as_neighbour(x, y, map){
                every_a.push(Pos {
                    x: x as i32, 
                    y: y as i32, 
                    z: 'a',
                    steps: 0, 
                    distance: 1
                })
            }
        }
    }
    return every_a;
}

fn a_star(start: Pos, goal: &Pos, map: &Vec<Vec<char>>) -> i32 {
    let mut visited: HashMap<String, i32> = HashMap::new();
    visited.insert(to_key(&start), 0);
    let mut queue: Vec<Pos> = vec![start];   
    while 0 < queue.len() {
        queue.sort_by(|a, b| (b.steps + b.distance).partial_cmp(&(a.steps + b.distance)).unwrap());
        let pos = queue.pop().unwrap();
        if pos.distance == 0 {
            return pos.steps;
        }

        for dxdy in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
            let new_pos: Pos = step(&pos, dxdy[0], dxdy[1], goal, map);
            if new_pos.distance == 404 {
                continue;
            }

            let key: String = to_key(&new_pos);
            let steps: i32 = new_pos.steps;
            
            if !visited.contains_key(&key) || visited.get(&key).unwrap() > &steps {
                visited.insert(key, steps);
                queue.push(new_pos);
            }
        }
    }
    return std::i32::MAX;
}

fn calculate_distance(x1: i32, y1: i32, z1: char, x2: i32, y2: i32, z2: char) -> i32 {
    let z1 = z1 as i32;
    let z2: i32= z2 as i32;
    let manhattan: i32 = (x1 - x2).abs() + 
                         (y1 - y2).abs() + 
                         (z1 - z2).abs();
    return manhattan;
}

fn step(pos: &Pos, dx: i32, dy: i32, goal: &Pos, map: &Vec<Vec<char>>) -> Pos {
    if valid_move(pos, dx, dy, map) {
        let x: i32 = pos.x + dx;
        let y: i32 = pos.y + dy;
        let z: char = map[y as usize][x as usize];

        Pos {
            x: x,
            y: y,
            z: z,
            steps: pos.steps + 1,
            distance: calculate_distance(x, y, z, goal.x, goal.y, goal.z)
        }
    }
    else {
        Pos {
            x: 404,
            y: 404,
            z: 'a',
            steps: 404,
            distance: 404
        }
    } 
}

fn valid_move(pos: &Pos, dx: i32, dy: i32, map: &Vec<Vec<char>>) -> bool {
    if pos.x + dx < 0 ||
       pos.x + dx >= map[0].len() as i32 {
        return false
       }
    if pos.y + dy < 0 ||
       pos.y + dy >= map.len() as i32 {
        return false
    }

    let x: usize = (pos.x + dx) as usize;
    let y: usize = (pos.y + dy) as usize;
    let z: char = map[y][x];
    
    let z1: i32 = pos.z as i32;
    let z2: i32 = z as i32;
    if z1+1 < z2 {
        return false;
    }

    return true;
}

fn to_key(pos: &Pos) -> String {
    format!("{},{},{}", pos.x, pos.y, pos.z)
}

fn b_as_neighbour(x: usize, y: usize, map: &Vec<Vec<char>>) -> bool { 
    if x > 0 && map[y][x-1] == 'b' || 
       y > 0 && map[y-1][x] == 'b' ||
       y+1 < map.len() && map[y+1][x] == 'b' ||
       x+1 < map[y].len() && map[y][x+1] == 'b' 
    {
        return true;
    }
    return false;
}

struct Pos {
    x: i32,
    y: i32,
    z: char,
    steps: i32,
    distance: i32,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}
