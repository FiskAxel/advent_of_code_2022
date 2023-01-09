use regex::Regex;
use std::{cmp, collections::HashMap};

fn main() {
    const ROW: i32 = 2000000;
    const MAX: i32 = 4000000;
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"\D+x=(-?\d+), y=(-?\d+)\D+x=(-?\d+), y=(-?\d+)").unwrap();

    let mut scanners: Vec<Scanner> = vec![];
    for i in &input {
        let caps = re.captures(i).unwrap();
        let pos: Point =     Point { x: caps[1].parse::<i32>().unwrap(), 
                                     y: caps[2].parse::<i32>().unwrap() };
        let closest: Point = Point { x: caps[3].parse::<i32>().unwrap(),
                                     y: caps[4].parse::<i32>().unwrap() };
        let reach: i32 = manhattan(&pos, &closest);
        let scanner: Scanner = Scanner { pos, closest, reach };
        scanners.push(scanner);
    }


    let mut beacons_at_row: HashMap<i32, i32> = HashMap::new();
    for scanner in &scanners {
        if scanner.closest.y == ROW {
            beacons_at_row.insert(scanner.closest.x, 1);
        }
    }
    let lines: Vec<Vec<i32>> = lines_in_row(&scanners, ROW);
    let sum: i32 = sum_of_lines(lines) - beacons_at_row.len() as i32;
    println!("Part 1: {}", sum);

    println!("Part 2: {}", part2(&scanners, MAX));
}

fn lines_in_row(scanners: &Vec<Scanner>, row: i32) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = vec![];
    let mut intersections: Vec<Vec<i32>> = vec![];

    for scanner in scanners {
        let dy: i32 = (row - &scanner.pos.y).abs();
        let dx: i32 = scanner.reach - dy;
        if dx < 0 { continue }

        let start: i32 = scanner.pos.x - dx;
        let end: i32 = scanner.pos.x + dx;
        let new_line: Vec<i32> = vec![start, end, 1];

        for l in &lines {
            let intersection: Option<Vec<i32>> = intersection(&new_line, l);
            match intersection {
                None => {},
                Some(i) => intersections.push(i)
            }
        }

        lines.push(new_line);
        lines.append(&mut intersections);
    }

    return lines
}

fn sum_of_lines(lines: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    for line in lines {
        sum += line[2] * (line[1] + 1 - line[0])
    }
    return sum;
}

fn intersection(a: &Vec<i32>, b: &Vec<i32>) -> Option<Vec<i32>> {
    if a[1] < b[0] || b[1] < a[0] {
        return None
    } 
    let start: i32 = cmp::max(a[0], b[0]);
    let end: i32 = cmp::min(a[1], b[1]);
    return Some(vec![start, end, -b[2]]);
}

fn manhattan(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn part2(scanners: &Vec<Scanner>, max: i32) -> i64 {
    for s1 in scanners {
        for s2 in scanners {
            let mut boundary_intersections:Vec<Point> = vec![];

            let a10: i32 = s1.pos.y-s1.pos.x - s1.reach-1;
            let a11: i32 = s1.pos.y-s1.pos.x + s1.reach+1;
            let b10: i32 = s1.pos.y+s1.pos.x - s1.reach-1;
            let b11: i32 = s1.pos.y+s1.pos.x + s1.reach+1;

            let a20: i32 = s2.pos.y-s2.pos.x - s2.reach-1;
            let a21: i32 = s2.pos.y-s2.pos.x + s2.reach+1;
            let b20: i32 = s2.pos.y+s2.pos.x - s2.reach-1;
            let b21: i32 = s2.pos.y+s2.pos.x + s2.reach+1;

            boundary_intersections.push(Point { x:(b20-a10)/2, y:(a10+b20)/2 });
            boundary_intersections.push(Point { x:(b21-a10)/2, y:(a10+b21)/2 });
            boundary_intersections.push(Point { x:(b20-a11)/2, y:(a11+b20)/2 });
            boundary_intersections.push(Point { x:(b21-a11)/2, y:(a11+b21)/2 });

            boundary_intersections.push(Point { x:(b10-a20)/2, y:(a20+b10)/2 });
            boundary_intersections.push(Point { x:(b11-a20)/2, y:(a20+b11)/2 });
            boundary_intersections.push(Point { x:(b10-a21)/2, y:(a21+b10)/2 });
            boundary_intersections.push(Point { x:(b11-a21)/2, y:(a21+b11)/2 });

            'p: for point in boundary_intersections {
                if point.x > max || point.y > max ||
                   point.x < 0 || point.y < 0 {
                    continue;
                }
                for scanner in scanners {
                    if manhattan(&point, &scanner.pos) <= scanner.reach {
                        continue 'p;
                    }
                }
                return (4000000 * point.x as i64) + point.y as i64;
            }   
        }
    }
    0
}

struct Scanner {
    pos: Point,
    closest: Point,
    reach: i32
}

struct Point { x: i32, y: i32 }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}
