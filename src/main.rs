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

type GitObject<'repo> = git2::Object<'repo>;

struct GitRange<'repo> {
    base_original: String,
    other_original: String,
    base: GitObject<'repo>,
    other: GitObject<'repo>,
}

impl<'repo> std::fmt::Display for GitRange<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*  let repo = match Repository::init(env::current_dir().unwrap()) {
            Ok(repo) => repo,
            Err(e) => panic!("Not inside a git repo: {}", e),
        }; */

        let base_hash = self.base.id();
        let other_hash = self.other.id();

        write!(
            f,
            "{}..{}\n{}..{}",
            self.base_original, self.other_original, base_hash, other_hash,
        )
    }
}

struct Stats {
    total_files_changed: usize,
    lines_added: usize,
    lines_deleted: usize,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} files changed: \n{} insertions\n{} deletions",
            self.total_files_changed, self.lines_added, self.lines_deleted,
        )
    }
}

fn argument_handling<'repo>(repo: &'repo git2::Repository) -> GitRange<'repo> {
    let mut args = env::args();

    // get the argument
    let other = args.nth(1);

    let other = match other {
        Some(other) => other,
        None => {
            fatal_exit("Need exactly one argument!");
            unreachable!();
        }
    };

    let base_original = "HEAD".into();
    let other_original = other.clone();

    let base = repo.revparse_single("HEAD").unwrap();
    let other = repo.revparse_single(&other).unwrap();

    GitRange {
        base_original,
        other_original,
        base,
        other,
    }
}

fn main() {
    // get string of cwd path
    let cwd = env::current_dir().unwrap();
    let full_path_string = cwd.join(""); // use full_pat_string.display() for print

    println!("debug: current directory is {}", full_path_string.display());

    // get repo obj
    let repo = match Repository::init(full_path_string) {
        Ok(repo) => repo,
        Err(e) => panic!("Not inside a git repo: {}", e),
    };

    let commit_range: GitRange = argument_handling(&repo);

    println!("{}", commit_range);
    //std::process::exit();
    let tree1 = commit_range.base.peel_to_tree().ok();
    let tree2 = commit_range.other.peel_to_tree().ok();

    assert!(tree1.is_some());
    assert!(tree2.is_some());

    let diff = repo.diff_tree_to_tree(tree1.as_ref(), tree2.as_ref(), None);

    let diffstat = diff.unwrap().stats().unwrap();
    //dbg!(diffstat);

    let diffstatsoptions = git2::DiffStatsFormat::all();

    // let stats = diffstats.to_buf();

    let final_stats: Stats = Stats {
        total_files_changed: diffstat.files_changed(),
        lines_added: diffstat.insertions(),
        lines_deleted: diffstat.deletions(),
    };

    println!("{}", final_stats);
}

// https://docs.rs/git2/0.12.0/git2/enum.DiffFormat.html name status

// https://docs.rs/git2/0.12.0/git2/struct.DiffStatsFormat.html  // diff --shortstat

fn bin_diff_info(repo: &Repository, base: &str, target: &str) {}
