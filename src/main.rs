use std::cmp::{max, min, Ordering};
use std::collections::BinaryHeap;
use std::env::args;
use std::fs::{self, DirEntry, ReadDir};

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u64,
    path: String,
}

impl FileInfo {
    fn new(file_name: String, file_size: u64, path: String) -> Self {
        FileInfo {
            name: file_name,
            size: file_size,
            path: path,
        }
    }

    fn pretty_print(&self) {
        println!("File Name: {}", self.name);
        println!("File Path: {}", self.path);
        println!(
            "File size: {}",
            Self::convert_bytes_to_pretty_string(self.size as f64)
        );
    }

    // pulled from https://gist.github.com/cjiali/b7a33d2e448bbfde381bc7c57682ee1a
    fn convert_bytes_to_pretty_string(num: f64) -> String {
        let negative: &str = if num.is_sign_positive() { "" } else { "-" };
        let num: f64 = num.abs();
        let units: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if num < 1_f64 {
            return format!("{}{} {}", negative, num, "B");
        }
        let delimiter: f64 = 1000_f64;
        let exponent: i32 = min(
            (num.ln() / delimiter.ln()).floor() as i32,
            (units.len() - 1) as i32,
        );
        let pretty_bytes: f64 = format!("{:.2}", num / delimiter.powi(exponent))
            .parse::<f64>()
            .unwrap()
            * 1_f64;
        let unit: &str = units[exponent as usize];
        format!("{}{} {}", negative, pretty_bytes, unit)
    }
}

impl Eq for FileInfo {}

impl Ord for FileInfo {
    fn cmp(&self, other: &FileInfo) -> Ordering {
        self.size.cmp(&other.size).reverse()
    }
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &FileInfo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &FileInfo) -> bool {
        self.size == other.size
    }
}

fn main() {
    let number_of_files: usize = parse_number_of_files();
    let dir: String = parse_dir();

    let mut heap: BinaryHeap<FileInfo> = BinaryHeap::new();
    recursively_inspect_sub_dirs(dir, &mut heap, number_of_files);

    for (i, file) in heap.into_sorted_vec().iter().enumerate() {
        println!("File Number: {}", i + 1);
        file.pretty_print();
        println!("---------------------------------------");
    }
}

fn parse_number_of_files() -> usize {
    if args().len() < 3 {
        println!("No number of files provided. Defaulting to 10.");
        10
    } else {
        let parsed = args().nth(2).unwrap().parse::<usize>();
        if parsed.is_ok() {
            parsed.unwrap()
        } else {
            10
        }
    }
}

fn parse_dir() -> String {
    if args().len() < 2 {
        println!("No directory provided. Defaulting to the current working directory.");
        String::from("./")
    } else {
        args().nth(1).unwrap()
    }
}

fn recursively_inspect_sub_dirs(
    dir: String,
    largest_file_heap: &mut BinaryHeap<FileInfo>,
    number_of_files: usize,
) {
    let paths: ReadDir = fs::read_dir(dir).unwrap();

    for path in paths {
        let path_name: String = String::from(path.as_ref().unwrap().path().to_str().unwrap());
        let file_name: String = path.as_ref().unwrap().file_name().into_string().unwrap();
        let is_dir: bool = path.as_ref().unwrap().metadata().unwrap().is_dir();
        let file_size: u64 = path.as_ref().unwrap().metadata().unwrap().len();
        if is_dir {
            recursively_inspect_sub_dirs(path_name, largest_file_heap, number_of_files);
        } else {
            let new_file: FileInfo = FileInfo::new(file_name, file_size, path_name);
            if largest_file_heap.len() >= number_of_files
                && new_file.size > largest_file_heap.peek().unwrap().size
            {
                largest_file_heap.pop();
            } else if largest_file_heap.len() < number_of_files {
                largest_file_heap.push(new_file)
            }
        }
    }
}
