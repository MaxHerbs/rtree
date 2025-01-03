use clap::Parser;
use std::path::{Path, PathBuf};
use std::{fs, usize};

/// Tree, implimented in rust
#[derive(Parser, Debug)]
#[command(author = "Your Name <your.email@example.com>", version = "1.0", about = "Does awesome things", long_about = None)]
struct Cli {
    /// Print the tree relative to a given path
    path: Option<String>,

    /// Maximum file depth to go into
    #[arg(short, long)]
    depth: Option<usize>,

    #[arg(short, long)]
    show_hidden: bool,
}

fn main() {
    // Parse the command-line arguments into the struct
    let args = Cli::parse();

    let path = match args.path {
        Some(path) => path,
        None => ".".to_string(),
    };

    run_tree(path, args.depth, args.show_hidden);
}

fn run_tree(target_path: String, depth: Option<usize>, show_hidden: bool) {
    let this_path = Path::new(&target_path);
    if !this_path.exists() {
        println!("Path: {:?} does not exist", this_path);
        std::process::exit(1);
    }

    tree(this_path, 0, depth, show_hidden);
}

fn tree(path: &Path, curr_depth: usize, max_depth: Option<usize>, show_hidden: bool) {
    if matches!(max_depth, Some(max) if curr_depth >= max) {
        return;
    }

    let this_dir = match fs::read_dir(path) {
        Ok(result) => result,
        Err(err) => return,
    };

    let mut files: Vec<_> = this_dir.filter_map(Result::ok).collect();
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

        println!(
            "{} {} {}",
            " ".repeat(2 * curr_depth),
            corner,
            file.file_name()
                .to_str()
                .expect("File name could not be parsed")
        );

        if file_path.is_dir() {
            tree(&file_path, curr_depth + 1, max_depth, show_hidden);
        }
    }
}
