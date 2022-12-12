use std::cell::RefCell;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    
    let monkeys: Vec<Monkey> = get_monkeys(&input);
    monkey_business(&monkeys, 20, true);
    println!("Part 1: {}", calculate_monkey_business_level(&monkeys));

    let monkeys: Vec<Monkey> = get_monkeys(&input);
    monkey_business(&monkeys, 10000, false);
    println!("Part 2: {}", calculate_monkey_business_level(&monkeys));
}

fn get_monkeys(input: &Vec<&str>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for i in input {
        let lines: Vec<&str> = i.lines().collect();

        let mut items: Vec<i64> = vec![];
        let mut split: Vec<&str> = lines[1].split("Starting items: ").collect();
        split = split[1].split(", ").collect();
        for num in split {
            let item = num.parse::<i64>().unwrap();
            items.push(item);
        }

        split = lines[2].split("Operation: new = old ").collect();
        split = split[1].split(" ").collect();
        let mut operator: char = split[0].chars().nth(0).unwrap();
        let num: i64;
        if split[1] == "old" {
            operator = '^';
            num = 2;
        } else {
            num = split[1].parse::<i64>().unwrap();
        }
        let operation = Operation { operator: operator, num: num };

        split = lines[3].split("Test: divisible by ").collect();    
        let divisor = split[1].parse::<i64>().unwrap();
        split = lines[4].split("If true: throw to monkey ").collect();
        let true_to = split[1].parse::<usize>().unwrap();
        split = lines[5].split("If false: throw to monkey ").collect();
        let false_to = split[1].parse::<usize>().unwrap();

        let monkey = Monkey {
            items: RefCell::new(items),
            operation: operation,
            divisor: divisor,
            true_to: true_to,
            false_to: false_to,
            count: RefCell::new(0)
        };
        monkeys.push(monkey);
    }
    return monkeys;
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: i32, part1: bool) {
    let cd = get_common_denominator(&monkeys);
    for _ in 0..rounds {
        for monkey in monkeys {
            let mut items = monkey.items.borrow_mut();
            items.reverse();
            while items.len() > 0 {
                let mut item = items.pop().unwrap();
                let op = &monkey.operation;
                match op.operator {
                    '+' => { item += op.num; },
                    '*' => { item *= op.num; },
                    '^' => { item *= item; },
                     _  => {}
                }
                item %= cd;

                if part1 {
                    item /= 3;
                }
                
                match item % monkey.divisor {
                    0 => {
                        monkeys[monkey.true_to].items.borrow_mut().push(item);
                    }
                    _ => {
                        monkeys[monkey.false_to].items.borrow_mut().push(item);
                    }
                }

                *monkey.count.borrow_mut() += 1;
            }
        }
    }
}

fn get_common_denominator(monkeys: &Vec<Monkey>) -> i64 {
    let mut cd: i64 = 1;
    for monkey in monkeys {
        let num = monkey.divisor;
        if cd % num != 0 {
            cd *= num;
        }
    }
    return cd;
}

fn calculate_monkey_business_level(monkeys: &Vec<Monkey>) -> i64 {
    let mut big1: i64 = 0;
    let mut big2: i64 = 0;
    for monkey in monkeys {
        let count: i64 = *monkey.count.borrow() as i64;
        if count > big1 {
            big2 = big1;
            big1 = count;
        } else if count > big2 {
            big2 = count;
        }
    }
    return big1 * big2;
}

struct Monkey {
    items: RefCell<Vec<i64>>,
    operation: Operation,
    divisor: i64,
    true_to: usize,
    false_to: usize,
    count: RefCell<i32>,
}

struct Operation {
    operator: char,
    num: i64,
}