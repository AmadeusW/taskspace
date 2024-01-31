use std::{fs};
use std::io::{Error, ErrorKind};
use uuid::Uuid;
use std::path::{Path, PathBuf};

const DATADIR: &str = ".taskspace";

fn main() {
    println!("Hello, world!");
}

fn getDataDirectory() -> Result<PathBuf, Error> {
    let currentDir = std::env::current_dir()
        .expect("Can't find current directory");
    return findPathInTree(currentDir, DATADIR);
}

fn findPathInTree(startDir: PathBuf, targetDir: &str) -> Result<PathBuf, Error> {
    loop {
        let mut currentPath = startDir;
        // Check if the target path is found
        let candidatePath = currentPath.join(DATADIR);
        if candidatePath.exists() {
            return Ok(candidatePath);
        }

        // Move up to the parent directory
        if !currentPath.pop() {
            // If we can't go up further, break the loop
            break;
        }
    }

    // If the target path is not found in the entire directory tree
    return Err(Error::new(ErrorKind::NotFound, "Data directory not found"));
}

fn createTask(alias: &str, dataDir: &PathBuf) -> Result<(), Error> {
    // Generate guid
    let id = Uuid::new_v4();
    // Create folder
    appendToIndex(alias, id, dataDir);
    return Ok(());
}

fn appendToIndex(alias: &str, id: &uuid::Uuid, dataDir: &PathBuf) -> Result<(), Error> {
    return Ok(());
}
