// execute with      cargo run --features "clippy"

extern crate git2;
use git2::Repository;
use std::env; // arg parsing // libgit

fn help() {
    println!("Usage: 'gitdiffbinstat [<commit/branch/tag/HEAD>]'");
    println!("Make sure to be inside a git repository!");
}

fn fatal_exit(msg: &str) {
    println!("Error: '{}'", msg);

    help();
    std::process::exit(1);
}

type GitObject = String;
fn argument_handling() -> GitObject {
    let mut args = env::args();

    // get the argument
    let obj = args.nth(1);

    let obj = match obj {
        Some(obj) => obj,
        None => {
            fatal_exit("Need exactly one argument!");
            unreachable!();
        }
    };

    obj as GitObject
}

fn main() {
    let gitobject: GitObject = argument_handling();

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
    let basecommit = "HEAD";
    let compare_against = "affb15fbac2c50dfdc4869253c07f299c13cd15c";

    let basecommit = repo.revparse_single(basecommit);
    let compare_against = repo.revparse_single(compare_against);

    //std::process::exit();
    let tree1 = basecommit.unwrap().peel_to_tree().ok();
    let tree2 = compare_against.unwrap().peel_to_tree().ok();

    //   dbg!((&tree1, &tree2));

    let diff = repo.diff_tree_to_tree(tree1.as_ref(), tree2.as_ref(), None);

    let diffstat = diff.unwrap().stats().unwrap();
    //dbg!(diffstat);

    let mut diffstatsoptions = git2::DiffStatsFormat::all();

    // let stats = diffstats.to_buf();

    println!(
        "{} files changed: \n{} insertions\n{} deletions",
        diffstat.files_changed(),
        diffstat.insertions(),
        diffstat.deletions()
    );
}

// https://docs.rs/git2/0.12.0/git2/enum.DiffFormat.html name status

// https://docs.rs/git2/0.12.0/git2/struct.DiffStatsFormat.html  // diff --shortstat

fn bin_diff_info(repo: &Repository, base: &str, target: &str) {}
