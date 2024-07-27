// use ferris_says::say;
// use std;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("watching {}", path);

    if let Err(e) = watch(path) {
        println!("error: {:?}", e);
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("changed: {:?}", event.paths),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}