// use ferris_says::say;
// use std;

pub mod watcher;

use watcher::{FsWatcher, Hasher};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("watching {}", path);

    let w: FsWatcher = FsWatcher::new(watcher::Hasher {}, path);

    if let Err(e) = w.watch() {
        println!("error: {:?}", e);
    }
}
