use std::{fs::read_to_string, path::{PathBuf, Path}, collections::HashMap};

fn main() {
    part_one();
    part_two();
}

struct FileEntry {
    path: PathBuf,
    size: usize,
}

fn parse_files(input: &str) -> Vec<FileEntry> {
    let mut entries: Vec<FileEntry> = Vec::new();
    let mut current_path = PathBuf::new();

    for line in input.lines() {
        let mut segments = line.split_whitespace();
        match segments.next() {
            Some("$") => match segments.next().expect("Empty command") {
                "cd" => match segments.next().expect("Missing directory for cd command") {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => current_path = PathBuf::from("/"),
                    directory_name => current_path.push(directory_name),
                },
                "ls" => {}
                command => panic!("Undefined command {command}"),
            },
            Some("dir") => {}
            Some(numeric_str) if numeric_str.chars().all(|chr| chr.is_ascii_digit()) => entries
                .push(FileEntry {
                    path: current_path.join(segments.next().expect("Could not find filename")),
                    size: numeric_str
                        .parse::<usize>()
                        .expect("Could not parse filesize"),
                }),
            None => break,
	    _ => panic!("Unexpected first segment")
        }
    }

    entries
}

fn part_one() {
    let data = read_to_string("data/07.txt").expect("No datafile");

    let entries = parse_files(&data);

    let mut directories: HashMap<&Path, usize> = HashMap::new();
    for file in &entries {
	for ancestor in file.path.ancestors().skip(1) {
	    *directories.entry(ancestor).or_insert(0) += file.size;
	}
    }

    let answer = directories.iter().filter_map(|(_dir, size)| {
	(*size <= 100_000).then_some(size)
    }).sum::<usize>();

    println!("{answer}")
}

fn part_two() {
    let data = read_to_string("data/07.txt").expect("No datafile");

    let fs_size = 70_000_000;
    let needed_space = 30_000_000;

    let entries = parse_files(&data);

    let mut directories: HashMap<&Path, usize> = HashMap::new();
    for file in &entries {
	for ancestor in file.path.ancestors().skip(1) {
	    *directories.entry(ancestor).or_insert(0) += file.size;
	}
    }

    let used_space = directories.get(&Path::new("/")).expect("Could not find size of root directory");
    let free_space = fs_size - used_space;

    let answer = directories.iter().filter_map(|(_dir, size)| {
	(*size >= needed_space - free_space).then_some(size)
    }).min().expect("Could not find optimal directory to delete");

    println!("{answer}")
}
