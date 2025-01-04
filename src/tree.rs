use colored::Colorize;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, usize};

static COLOR_MAP: Lazy<HashMap<&'static str, (u8, u8, u8)>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("py", (0, 0, 255));
    map.insert("js", (255, 255, 0));
    map.insert("rs", (255, 0, 0));
    map
});

enum FileType {
    File,
    Dir,
}

pub fn tree(path: &Path, curr_depth: usize, max_depth: Option<usize>, show_hidden: bool) {
    if matches!(max_depth, Some(max) if curr_depth >= max) {
        return;
    }

    let this_dir = match fs::read_dir(path) {
        Ok(result) => result,
        Err(_err) => return,
    };

    let files: Vec<_> = this_dir.filter_map(Result::ok).collect();
    let num_files = files.len();

    for (i, file) in files.iter().enumerate() {
        let file_path = file.path();

        if !show_hidden {
            if let Some(file_name) = file.file_name().to_str() {
                if file_name.chars().next() == Some('.') {
                    continue;
                }
            }
        }

        let is_last = i == num_files - 1;
        let corner = if is_last { "└" } else { "├" };

        let file_type = if file_path.is_dir() {
            FileType::Dir
        } else {
            FileType::File
        };

        print_text(
            curr_depth,
            corner,
            file.file_name()
                .to_str()
                .expect("File name could not be parsed"),
            file_type,
        );

        if file_path.is_dir() {
            tree(&file_path, curr_depth + 1, max_depth, show_hidden);
        }
    }
}

fn print_text(curr_depth: usize, corner: &str, file_name: &str, file_type: FileType) {
    let formatted_text = format_text(file_name, file_type);

    println!(
        "{} {} {}",
        " ".repeat(2 * curr_depth),
        corner,
        formatted_text
    );
}

fn format_text(file_name: &str, file_type: FileType) -> String {
    let (r, g, b) = match file_to_color(file_name, &file_type) {
        Some((r, g, b)) => (r, g, b),
        None => (255, 255, 0),
    };

    match file_type {
        FileType::File => file_name.truecolor(r, g, b).to_string(),
        FileType::Dir => file_name.truecolor(r, g, b).italic().to_string(),
    }
}

fn file_to_color(file_name: &str, file_type: &FileType) -> Option<(u8, u8, u8)> {
    match file_type {
        FileType::Dir => Some((255, 255, 255)),
        FileType::File => color_file_type(file_name),
    }
}

fn color_file_type(file_name: &str) -> Option<(u8, u8, u8)> {
    if !file_name.contains('.') {
        return None;
    }

    let file_components: Vec<&str> = file_name.split('.').collect();
    if let Some(&extension) = file_components.last() {
        return Some(*COLOR_MAP.get(extension).unwrap_or(&(255, 255, 255)));
    }
    None
}
