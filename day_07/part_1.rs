use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::{Rc, Weak};

struct Dir {
    files: HashMap<String, u64>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    parent: Weak<RefCell<Dir>>
}

fn dir_sum(dir: &Dir, small_dir_sum: &mut u64) -> u64 {
    let mut dir_size = 0;
    for (_, file_size) in &dir.files {
        dir_size += file_size;
    }

    for (_, sub_d) in &dir.sub_dirs {
        dir_size += dir_sum(&sub_d.borrow(), small_dir_sum);
    }

    if dir_size <= 100000 {
        *small_dir_sum += dir_size;
    }

    return dir_size;
}

fn main() {
    let stdin = std::io::stdin();

    let root = Rc::new_cyclic(|r| {
        RefCell::new(Dir {
            files: HashMap::new(),
            sub_dirs: HashMap::new(),
            parent: r.clone()
        })
    });
    let mut curr_dir = root.clone();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.starts_with("$ cd") {
            match line.split_whitespace().last().unwrap() {
                "/" => curr_dir = root.clone(),
                ".." => {
                    let new_dir = curr_dir.borrow_mut().parent.upgrade().unwrap();
                    curr_dir = new_dir;
                }
                sub_d => {
                    let new_dir = curr_dir.borrow_mut()
                        .sub_dirs
                        .entry(sub_d.to_string())
                        .or_insert(Rc::new(RefCell::new(Dir {
                            files: HashMap::new(),
                            sub_dirs: HashMap::new(),
                            parent: Rc::downgrade(&curr_dir)
                        }))).clone();
                    curr_dir = new_dir;
                }
            }
        }
        else if line.starts_with("$ ls") {}
        else {
            let words: Vec<_> = line.split_whitespace().collect();
            if words[0] == "dir" {
                curr_dir.borrow_mut()
                    .sub_dirs
                    .insert(words[1].to_string(), Rc::new(RefCell::new(Dir {
                        files: HashMap::new(),
                        sub_dirs: HashMap::new(),
                        parent: Rc::downgrade(&curr_dir)
                    })));
            }
            else {
                curr_dir.borrow_mut()
                    .files
                    .insert(words[1].to_string(), words[0].parse().unwrap());
            }
        }
    }

    let mut small_dir_sum = 0;
    dir_sum(&root.borrow(), &mut small_dir_sum);
    println!("{small_dir_sum}");
}
