use std::{fs::{self}, path::Path, println};
use rayon::prelude::*;

fn main() {
    let mut args = std::env::args();

    let program = args.next().unwrap();
    let search_dir = args.next().unwrap();
    let search_term = args.next().unwrap();

    list_files_like(Path::new(&search_dir), &search_term);
}

fn list_files_like(search_dir: &Path, search_term: &str) {
    let entries: Vec<_> = match fs::read_dir(search_dir) {
        Ok(entries) => entries.filter_map(Result::ok).collect(),
        Err(_) => return,
    };

    entries.par_iter().for_each(|entry| {
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            let file_name = entry.file_name();

            if file_name.to_string_lossy().contains(search_term) {
                println!("{}", entry.path().display());
            }
        } else if file_type.is_dir() {
            list_files_like(&entry.path(), search_term);
        }
    });
}