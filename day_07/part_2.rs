use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::{Rc, Weak};

struct Dir {
    files: HashMap<String, u64>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    parent: Weak<RefCell<Dir>>,
    size: u64
}

fn populate_size(dir: &mut Dir) -> u64 {
    let mut dir_size = 0;
    for (_, file_size) in &dir.files {
        dir_size += file_size;
    }

    for (_, sub_d) in &dir.sub_dirs {
        dir_size += populate_size(&mut sub_d.borrow_mut());
    }

    dir.size = dir_size;

    return dir_size;
}

fn dir_size_to_remove(dir: &Dir, target: u64, best_so_far: &mut u64) {
    if dir.size >= target {
        *best_so_far = (*best_so_far).min(dir.size);
        for (_, sub_d) in &dir.sub_dirs {
            dir_size_to_remove(&sub_d.borrow(), target, best_so_far);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let root = Rc::new_cyclic(|r| {
        RefCell::new(Dir {
            files: HashMap::new(),
            sub_dirs: HashMap::new(),
            parent: r.clone(),
            size: 0
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
                            parent: Rc::downgrade(&curr_dir),
                            size: 0
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
                        parent: Rc::downgrade(&curr_dir),
                        size: 0
                    })));
            }
            else {
                curr_dir.borrow_mut()
                    .files
                    .insert(words[1].to_string(), words[0].parse().unwrap());
            }
        }
    }

    populate_size(&mut root.borrow_mut());
    let mut to_remove_size = u64::MAX;
    dir_size_to_remove(&root.borrow(), 30000000 - (70000000 - root.borrow().size), &mut to_remove_size);
    println!("{to_remove_size}");
}
