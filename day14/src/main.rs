fn main() {
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<&str> = input.trim().lines().collect();
    let mut rock_paths: Vec<Vec<Point>> = vec![];
    for line in input {
        let mut rock_path: Vec<Point> = vec![];
        let cordinates: Vec<&str> = line.split(" -> ").collect();
        for c in cordinates {
            let xy: Vec<i32> = c.split(",")
                .map(|a| a.parse::<i32>().unwrap())    
                .collect();
            let point: Point = Point { x:xy[0], y:xy[1] };
            rock_path.push(point);
        }
        rock_paths.push(rock_path);
    }
    
    let max_y: i32 = get_max_y(&rock_paths);
    let max_x: i32 = get_max_x(&rock_paths);
    let min_x: i32 = get_min_x(&rock_paths);

    let mut rocks: Vec<Point> = vec![];
    for rock in rock_paths {
        for i in 1..rock.len() {
            let y1: i32 = rock[i-1].y;
            let y2: i32 = rock[i].y;
            for y in min(y1, y2)..max(y1, y2)+1 {
                let x1: i32 = rock[i-1].x;
                let x2: i32 = rock[i].x;
                for x in min(x1, x2)..max(x1, x2)+1 {
                    let point: Point = Point { x: x, y: y };
                    if !rocks.contains(&point) {
                        rocks.push(point);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", poor_sand(&rocks, max_y, false)); // runs in 4 seconds
    //println!("Part 2: {}", poor_sand(&rocks, max_y, true)); // runs in an hour
    println!("Part 2: {}", part_2(&rocks, max_y, max_x, min_x)); // runs in 8 seconds
}


fn poor_sand(rocks: &Vec<Point>, max_y: i32, part2: bool) -> usize {
    let mut still_sand: Vec<Point> = vec![];
    'a: loop {
        let mut sand: Point = Point { x: 500, y: 0 };
        loop {
            sand.y += 1;
            if blocked(&sand, &still_sand, &rocks, part2, max_y) {
                sand.x -= 1;
                if blocked(&sand, &still_sand, &rocks, part2, max_y) {
                    sand.x += 2;
                    if blocked(&sand, &still_sand, &rocks, part2, max_y) {
                        sand.x -= 1;
                        sand.y -= 1;
                        if sand.x == 500 && sand.y == 0 {
                            still_sand.push(sand);
                            break 'a;
                        }
                        still_sand.push(sand);
                        break;
                    }
                }
            }
            if sand.y > max_y {
                break 'a;
            } 
        }
    }
    write_sandcastle(&still_sand, &rocks, max_y);
    return still_sand.len();
}

fn blocked(sand: &Point, still_sand: &Vec<Point>, rocks: &Vec<Point>, part2: bool, max_y: i32) -> bool {
    if still_sand.contains(sand) || rocks.contains(sand) {
        return true
    }
    if part2 && sand.y == max_y {
        return true
    }
    return false
}

fn get_max_y(paths: &Vec<Vec<Point>>) -> i32 {
    let mut deepest = 0;
    for rocks in paths {
        for rock in rocks {
            if deepest < rock.y {
                deepest = rock.y;
            } 
        }
    }
    return deepest + 2
}
fn get_max_x(paths: &Vec<Vec<Point>>) -> i32 {
    let mut highest = 0;
    for rocks in paths {
        for rock in rocks {
            if highest < rock.x {
                highest = rock.x;
            } 
        }
    }
    return highest + 2
}
fn get_min_x(paths: &Vec<Vec<Point>>) -> i32 {
    let mut lowest = std::i32::MAX;
    for rocks in paths {
        for rock in rocks {
            if lowest > rock.x {
                lowest = rock.x;
            } 
        }
    }
    return lowest - 2
}


fn part_2(rocks: &Vec<Point>, max_y: i32, max_x: i32, min_x: i32) -> i32 {
    let source: Point = Point { x: 500, y: 0 };
    let mut sand: Vec<Point> = vec![source];
    let mut empty: Vec<Point> = vec![];
    for y in 1..max_y {
        for x in max(500-y, min_x)..min(501+y, max_x) {
            let point: Point = Point { x: x, y: y };
            if rocks.contains(&point) {
                continue;
            }

            if sand_above(&point, &sand) {
                sand.push(point);
            } else {
                empty.push(point);
            }
        }
    }

    write_sandcastle(&sand, rocks, max_y);
    let max_sand: i32 = (max_y).pow(2);
    max_sand - rocks.len() as i32 - empty.len() as i32
    
}

fn sand_above(point: &Point, sand: &Vec<Point>) -> bool {
    let x: i32 = point.x;
    let y: i32 = point.y;
    let up: Point = Point { x: x, y: y-1 };
    let up_left: Point = Point { x: x-1, y: y-1 };
    let up_right: Point = Point { x: x+1, y: y-1 };
    if sand.contains(&up) ||
       sand.contains(&up_left) ||
       sand.contains(&up_right) {
        true
    } else {
        false
    }
}

struct Point { x: i32, y: i32 }
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}
fn min(a: i32, b: i32) -> i32 {
    if a < b { a }
    else { b }
}
fn max(a: i32, b: i32) -> i32 {
    if a > b { a }
    else { b }
}



use std::io::Write;
fn write_sandcastle(sand: &Vec<Point>, rocks: &Vec<Point>, max_y: i32) {
    let mut string: String = String::from("");
    for y in 0..max_y {
        for x in 450..550 {
            let p = Point { x: x, y: y };
            if sand.contains(&p) {
                string += "o";
            } else if rocks.contains(&p) {
                string += "#";
            } else {
                string += ".";
            }
        }
        string += "\r\n";
    }

    let mut file = std::fs::File::create("src/out.txt").expect("create failed");
    file.write_all(string.as_bytes()).expect("write failed");
}