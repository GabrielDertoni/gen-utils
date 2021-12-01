#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::path::PathBuf;
use std::fs;
use std::collections::VecDeque;

use genutils::yield_from;

#[genutils::generator]
fn dfs_traverse_files(root: PathBuf) -> impl Generator<Yield = PathBuf> {
    if root.is_dir() {
        let dir_iter = fs::read_dir(&root)
            .unwrap()
            .filter_map(Result::ok)
            .map(|d| d.path());

        for subpath in dir_iter {
            yield_from!(dfs_traverse_files(subpath));
        }
    }

    yield root;
}

#[genutils::generator]
fn bfs_traverse_files(root: PathBuf) -> impl Generator<Yield = PathBuf> {
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while let Some(path) = queue.pop_front() {
        if path.is_dir() {
            let dir_iter = fs::read_dir(path)
                .unwrap()
                .filter_map(Result::ok)
                .map(|d| d.path());

            queue.extend(dir_iter);
        } else {
            yield path;
        }
    }
}

fn main() {

    let files_gen = bfs_traverse_files("/home".into());

    for file in files_gen {
        println!("File name: {:?}", file.as_path());
    }

}