#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// execute with      cargo run --features "clippy"

extern crate git2;
use std::env;  // arg parsing
use git2::Repository; // libgit


fn help() {
    println!("Usage: 'gitdiffbinstat [<commit/branch/tag/HEAD>]'");
    println!("Make sure to be inside a git repository!");
}

fn fatal_exit(msg: &str) {
    println!("Error:");
    println!("{}", msg);
    panic!("bla");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2  { // one arg passed
        println!("arg passed");
    } else {
        let msg = "no args passed".to_string();
        fatal_exit(&msg);
    }

    help();
    // get string of cwd path
    let cwd = env::current_dir().unwrap();
    let full_path_string = cwd.join(""); // use full_pat_string.display() for print

    println!("debug: current directory is {}", full_path_string.display());

    // get repo obj
    let repo = match Repository::init(full_path_string) {
        Ok(repo) => repo,
        Err(e) => panic!("Not inside a git repo: {}", e),
    };

    // get the input commit
    let basecommit = "rust";
    let compare_against = "abf184d48c6230989e5d574a5588f079cd075a35";

    let basecommit = repo.revparse_single(basecommit);
    let compare_against = repo.revparse_single(compare_against);

    //let mut diffoptions = git2::DiffOptions::new();

    let diff = repo.diff_index_to_index(&(basecommit.unwrap()), &compare_against.unwrap(), None);
 
}


