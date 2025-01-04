mod tree;
use clap::Parser;
use std::path::Path;

/// Tree, implimented in rust
#[derive(Parser, Debug)]
#[command(author = "Your Name <your.email@example.com>", version = "1.0", about = "Does awesome things", long_about = None)]
struct Cli {
    /// Print the tree relative to a given path
    path: Option<String>,

    /// Maximum file depth to go into
    #[arg(short, long)]
    depth: Option<usize>,

    /// Show hidden files
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
    tree::tree(this_path, 0, depth, show_hidden);
}
