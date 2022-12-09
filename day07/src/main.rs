use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut input: Vec<&str> = input
        .trim()
        .lines()
        .collect();
    input.remove(0); // "$ cd /"

    let root = Rc::new(Dir::new( None));
    let mut current = Rc::clone(&root);
    for line in input {
        let instruction: Vec<&str> = line.split(' ').collect();
        match (instruction[0], instruction[1]) {
            ("$", "ls") => { continue }
            ("$", "cd") => {
                match instruction[2] {
                    ".." => {
                        current = Rc::clone(current.parent.as_ref().unwrap());
                    }
                    name => {
                        let dir = current.directories.borrow().get(name).unwrap().clone();
                        current = dir;
                    }
                }
            }
            ("dir", name) => {
                let name = String::from(name);
                let parent = Some(Rc::clone(&current));
                let dir = Rc::new(Dir::new(parent));
                current.directories.borrow_mut().insert(name, dir);
            }
            (size, _) => {
                let size = size.parse::<u32>().unwrap();
                *current.size.borrow_mut() += size;
            }
        }
    }

    calculate_dir_sizes(root.clone());

    let mut sum: u32 = 0;
    let mut sizes: Vec<u32> = vec![];
    let mut queue = vec![root.clone()];
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        let size = *current.size.borrow();
        
        if size < 100000 {
            sum += size;
        }
        
        for dir in current.directories.borrow().values() {
            queue.push(dir.clone());
        }
        sizes.push(size);
    } 

    println!("Part 1: {}", sum);

    let free_space: u32 = 70000000 - *root.size.borrow();
    let needed_space: u32 = 30000000 - free_space;
    sizes.sort();
    for s in sizes {
        if s >= needed_space {
            println!("Part 2: {}", s);
            break;
        }
    }

}

fn calculate_dir_sizes(dir: Rc<Dir>) -> u32 {
    for d in dir.directories.borrow_mut().values() {
        *dir.size.borrow_mut() += calculate_dir_sizes(Rc::clone(d));
    }
    return *dir.size.borrow();
}

struct Dir {
    size: RefCell<u32>,
    parent: Option<Rc<Dir>>,
    directories: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    pub fn new(parent: Option<Rc<Dir>>) -> Self {
        Self {
            size: RefCell::new(0),
            parent,
            directories: RefCell::new(HashMap::new()),
        }
    }
}