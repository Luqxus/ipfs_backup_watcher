pub mod watcher;

use std::collections::HashMap;

struct FsWatcher {}

impl FsWatcher {
    pub fn new() -> &FsWatcher {
        return &FsWatcher {};
    }

    pub fn delete(&mut self, paths: &[&str]) -> Result<Ok(), Err(&str)> {}
}

struct HashLog {
    log: &HashMap,
}

impl HashLog {}
