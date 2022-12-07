#![feature(let_chains)]

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap());

    let root_directory = Rc::new(RefCell::new(VirtualDirectory {
        name: String::new(),
        parent: None,
        children: Vec::new(),
    }));

    let mut current_directory = root_directory.clone();

    for line in lines {
        if line.starts_with('$') {
            let parts = line.split(' ').collect::<Vec<_>>();

            if parts[1] == "cd" {
                if parts[2] == "/" {
                    current_directory = root_directory.clone();
                } else if parts[2] == ".." {
                    let parent_directory =
                        current_directory.borrow().parent.as_ref().unwrap().clone();

                    current_directory = parent_directory;
                } else {
                    let child_directory = current_directory
                        .borrow()
                        .children
                        .iter()
                        .find_map(|child| match child {
                            Entry::Directory(directory) => {
                                if parts[2] == directory.borrow().name {
                                    return Some(directory.clone());
                                }

                                None
                            }
                            _ => None,
                        })
                        .unwrap();

                    current_directory = child_directory;
                }
            }
        } else {
            let parts = line.split(' ').collect::<Vec<_>>();

            let name = parts[1].to_owned();

            if parts[0] == "dir" {
                current_directory
                    .borrow_mut()
                    .children
                    .push(Entry::Directory(Rc::new(RefCell::new(VirtualDirectory {
                        name,
                        parent: Some(current_directory.clone()),
                        children: Vec::new(),
                    }))));
            } else {
                let size = parts[0].parse::<usize>().unwrap();

                current_directory
                    .borrow_mut()
                    .children
                    .push(Entry::File(VirtualFile { name, size }));
            }
        }
    }

    let size = root_directory
        .borrow()
        .directories()
        .into_iter()
        .map(|directory| directory.borrow().size())
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    println!("{size}");
}

pub enum Entry {
    Directory(Rc<RefCell<VirtualDirectory>>),
    File(VirtualFile),
}

pub struct VirtualDirectory {
    name: String,
    parent: Option<Rc<RefCell<VirtualDirectory>>>,
    children: Vec<Entry>,
}

impl VirtualDirectory {
    pub fn size(&self) -> usize {
        let mut size = 0;

        for child in &self.children {
            size += match child {
                Entry::Directory(directory) => directory.borrow().size(),
                Entry::File(file) => file.size,
            };
        }

        size
    }

    pub fn directories(&self) -> Vec<Rc<RefCell<VirtualDirectory>>> {
        let mut directories = Vec::new();
        let mut queue = VecDeque::new();

        for child in &self.children {
            if let Entry::Directory(directory) = child {
                directories.push(directory.clone());
                queue.push_back(directory.clone());
            }
        }

        while let Some(root) = queue.pop_front() {
            for child in &root.borrow().children {
                if let Entry::Directory(directory) = child {
                    directories.push(directory.clone());
                    queue.push_back(directory.clone());
                }
            }
        }

        directories
    }
}

pub struct VirtualFile {
    name: String,
    size: usize,
}
