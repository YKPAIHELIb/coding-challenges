use std::io::{self, BufRead};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use anyhow::{Result, anyhow, Ok};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let commands = parse_input(lines).unwrap();
    let root = build_file_tree(commands);

    part_two(root);
}

fn part_one(root: Rc<RefCell<Directory>>) {
    let mut total_size: u32 = 0;
    calc_size(&root.borrow(), &mut total_size);

    println!("Total size is {total_size}");

    fn calc_size(dir: &Directory, total_size: &mut u32) -> u32 {
        let mut dir_size: u32 = dir.files.iter().map(|f| f.size).sum();
        for sub_dir in dir.sub_directories.iter().map(|x| x.borrow()) {
            dir_size += calc_size(&sub_dir, total_size);
        }
        
        if dir_size <= 100000 {
            *total_size += dir_size;
        }
        dir_size
    }
}

fn part_two(root: Rc<RefCell<Directory>>) {
    let mut all_sizes = vec![];
    let used_space = calculate_and_fill_sizes(&root.borrow(), &mut all_sizes);
    let unused_space = 70_000_000 - used_space;
    let space_to_free_up = 30_000_000 - unused_space;
    let size_to_delete = all_sizes.iter()
        .filter(|&x| x > &space_to_free_up)
        .min().unwrap();

    println!("Directory to delete has size: {size_to_delete}");

    fn calculate_and_fill_sizes(dir: &Directory, all_sizes: &mut Vec<u32>) -> u32 {
        let mut dir_size: u32 = dir.files.iter().map(|f| f.size).sum();
        for sub_dir in dir.sub_directories.iter().map(|x| x.borrow()) {
            dir_size += calculate_and_fill_sizes(&sub_dir, all_sizes);
        }

        all_sizes.push(dir_size);
        dir_size
    }
}

fn parse_input(lines: io::Lines<io::StdinLock<'_>>) -> Result<Vec<CommandWithOutput>> {
    let mut lines_iter = lines.into_iter();
    let mut all_commands: Vec<CommandWithOutput> = Vec::new();
    let mut current_command: Option<CommandWithOutput> = None;
    while let Some(line) = lines_iter.next() {
        let line = line?;
        if line.starts_with('$') {
            if let Some(command) = current_command {
                all_commands.push(command);
            }
            current_command = Some(parse_command(&line)?);
        }
        else if let Some(CommandWithOutput::Ls(ls_command)) = &mut current_command {
            match line {
                l if l.starts_with("dir") => ls_command.directories.push(l[4..].to_owned()),
                l if l.chars().next().unwrap().is_ascii_digit() => {
                    let (size_str, file_name) = l.split_once(' ').unwrap();
                    ls_command.files.push(File {
                        name: file_name.to_owned(),
                        size: size_str.parse().unwrap(),
                    })
                },
                _ => panic!()
            }
        }
    }

    if let Some(command) = current_command {
        all_commands.push(command);
    }

    return Ok(all_commands);
    
    fn parse_command(line: &str) -> Result<CommandWithOutput> {
        let line = &line[2..];
        let command = match line {
            "cd /" => CommandWithOutput::Cd(CdCommand::Root),
            "cd .." => CommandWithOutput::Cd(CdCommand::Up),
            cmd if cmd.starts_with("cd ") => {
                let dir_name = &cmd[3..];
                CommandWithOutput::Cd(CdCommand::ToDirectory(dir_name.to_owned()))
            },
            "ls" => CommandWithOutput::Ls(LsCommand::empty()),
            _ => return Err(anyhow!("Not supported command")),
        };
        Ok(command)
    }
}

fn build_file_tree(commands: Vec<CommandWithOutput>) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::parent("/".to_owned())));
    let mut current_directory = root.clone();
    for command in commands {
        match command {
            CommandWithOutput::Cd(CdCommand::Root) => current_directory = root.clone(),
            CommandWithOutput::Cd(CdCommand::Up) => {
                let temp = current_directory;
                current_directory = temp.borrow() 
                    .parent.as_ref().unwrap()
                    .upgrade().unwrap();
            },
            CommandWithOutput::Cd(CdCommand::ToDirectory(dir_name)) => {
                let temp = current_directory;
                current_directory = temp.borrow().sub_directories
                    .iter()
                    .find(|&dir| dir.borrow().name.eq(&dir_name))
                    .unwrap()
                    .clone();
            },
            CommandWithOutput::Ls(LsCommand { directories, files }) => {
                let mut directory = current_directory.borrow_mut();
                directory.files = files;
                directory.sub_directories = directories.into_iter()
                    .map(|name| Rc::new(RefCell::new(Directory::new(name, Rc::downgrade(&current_directory)))))
                    .collect();
            }
        }
    }

    root
}

enum CommandWithOutput {
    Cd(CdCommand),
    Ls(LsCommand),
}

enum CdCommand {
    Root,
    Up,
    ToDirectory(String),
}

struct LsCommand {
    directories: Vec<String>,
    files: Vec<File>,
}

impl LsCommand {
    fn empty() -> Self {
        LsCommand { directories: Vec::new(), files: Vec::new() }
    }
}

struct File {
    name: String,
    size: u32,
}

struct Directory {
    name: String,
    sub_directories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
    parent: Option<Weak<RefCell<Directory>>>,
}

impl Directory {
    fn parent(name: String) -> Self {
        Directory { 
            name,
            sub_directories: vec![],
            files: vec![],
            parent: None,
        }
    }

    fn new(name: String, parent: Weak<RefCell<Directory>>) -> Self {
        Directory { 
            name,
            sub_directories: vec![],
            files: vec![],
            parent: Some(parent),
        }
    }
}
