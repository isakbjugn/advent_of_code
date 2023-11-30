use std::{fmt, rc::Rc};
use std::cell::{Ref, RefCell};

struct TreeNode {
    pub name: String,
    pub size: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(str: &str) -> TreeNode {
        TreeNode {
            name: String::from(str),
            size: None,
            children: vec![],
            parent: None
        }
    }

    pub fn get_content_size(&self) -> u32 {
        match self.size {
            Some(size) => size,
            None => self.children.iter().fold(0, |acc, n| acc + n.borrow().get_content_size())
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }

    pub fn register_dir_sizes(&self, dir_sizes: &mut Vec<(String, u32)>) {
        if self.children.is_empty() {
            return
        }
        dir_sizes.push((self.name.clone(), self.get_content_size()));
        for child in self.children.iter() {
            child.borrow().register_dir_sizes(dir_sizes);
        }
    }
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut info = String::new();
        if let Some(s) = &self.size {
            info = info + "file, size=" + &*s.to_string();
        } else {
            info = String::from("dir");
        }
        if self.children.is_empty() {
            write!(f, "– {} ({})", &self.name, info)
        } else {
            write!(f, "– {} ({})\n {:#?}", &self.name, info, &self.children.iter().map(|n| n.borrow()).collect::<Vec<Ref<TreeNode>>>())
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let root = init_tree(input);
    //println!("{:#?}", root.borrow());
    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    root.borrow().register_dir_sizes(&mut dir_sizes);

    const MAX_DIR_SIZE: u32 = 100000;
    let small_dirs = dir_sizes.into_iter().filter(|(_, val)| *val <= MAX_DIR_SIZE);
    small_dirs.into_iter().map(|(_, val)| val).collect::<Vec<u32>>().iter().sum()
}

fn init_tree(input: &str) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new("root")));
    let mut current = Rc::clone(&root);
    let mut reading_dir_content = false;

    for line in input.lines() {
        if line.contains('$') {
            reading_dir_content = false;
        }
        if reading_dir_content {
            let dir_content: Vec<&str> = line.split(' ').collect();
            let (prefix, name) = (dir_content[0], dir_content[1]);
            let child = Rc::new(RefCell::new(TreeNode::new(name)));
            current.borrow_mut().add_child(Rc::clone(&child));

            {
                let mut mut_child = child.borrow_mut();
                match prefix {
                    s if s != "dir" => mut_child.size = Some(s.parse::<u32>().unwrap()),
                    _ => ()
                }
                mut_child.parent = Some(Rc::clone(&current));
            }
            continue
        }
        match line {
            "$ ls" => reading_dir_content = true,
            "$ cd /" => current = Rc::clone(&root),
            "$ cd .." => {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                },
            x => {
                let command: Vec<&str> = x.split(' ').collect();
                let dir = command.into_iter().last().unwrap();
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().children.iter().filter(|n| n.borrow().name == dir).collect::<Vec<&Rc<RefCell<TreeNode>>>>()[0] );
            }
        }
    }
    root
}

pub fn part_2(input: &str) -> u32 {
    let root = init_tree(input);
    //println!("{:#?}", root.borrow());
    let mut dir_sizes: Vec<(String, u32)> = Vec::new();
    root.borrow().register_dir_sizes(&mut dir_sizes);

    let min_dir_size: u32 = root.borrow().get_content_size() - 40000000;
    let large_dirs = dir_sizes.into_iter().filter(|(_, val)| *val >= min_dir_size);
    let large_dir_sizes = large_dirs.into_iter().map(|(_, val)| val);
    large_dir_sizes.into_iter().min().unwrap()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_1(input), 95437)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_2(input), 24933642)
}
