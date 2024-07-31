use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};

pub struct FsWatcher {
    path: String,
    hasher: Hasher,
}

impl FsWatcher {
    pub fn new(hasher: Hasher, path: String) -> FsWatcher {
        FsWatcher { path, hasher }
    }

    pub fn watch(&self) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        watcher.watch(self.path.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    println!("changed: {:?}", event.paths);
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }

    fn updateHashMap() {}
}

pub struct Hasher {}

impl Hasher {
    pub fn hash(file: &str) -> Result<String> {
        let mut file = File::open(file)?;

        let mut hasher = Sha256::new();

        let mut buffer = [0; 4096];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}
