use std::fs;
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;
use std::path::PathBuf;
use std::collections::HashMap;

const DATADIR: &str = ".taskspace";
const INDEXFILE: &str = "index.txt";

fn main() {
    println!("Hello, world!");
}

fn getDataDirectory() -> Option<PathBuf> {
    let currentDir = std::env::current_dir()
        .expect("Expected to find the current directory");
    return findPathInTree(&currentDir, DATADIR);
}

fn findPathInTree(startDir: &PathBuf, targetDir: &str) -> Option<PathBuf> {
    loop {
        let mut currentPath = startDir.clone();
        // Check if the target path is found
        let candidatePath = currentPath.join(DATADIR);
        if candidatePath.exists() {
            return Some(candidatePath);
        }

        // Move up to the parent directory
        if !currentPath.pop() {
            // If we can't go up further, break the loop
            break;
        }
    }

    // If the target path is not found in the entire directory tree
    None
}

fn createTask(alias: &str, dataDir: &PathBuf) -> Result<(), &'static str> {
    let id = Uuid::new_v4();

    let taskPath = dataDir.join(format!("{}", id));
    fs::create_dir_all(&taskPath);

    appendToIndex(alias, &id, dataDir);
    return Ok(());
}

fn appendToIndex(alias: &str, id: &uuid::Uuid, dataDir: &PathBuf) -> Result<(), &'static str> {
    let indexPath = dataDir.join(INDEXFILE);
    let mut file = fs::File::create(&indexPath)
        .expect(&format!("Expected to access the index file {}", indexPath.display()));
    writeln!(&mut file, "{alias}:{id}");
    return Ok(());
}

fn getIdFromIndex(alias: &str, dataDir: &PathBuf) -> Option<Uuid> {// Result<Uuid, String> {
    let indexPath = dataDir.join(INDEXFILE);
    let file = fs::File::open(&indexPath)
        .expect(&format!("Expected to open the index file {}", indexPath.display()));
    let reader = BufReader::new(file);
    
    // Iterate over lines in the file
    for line in reader.lines() {
        let content = line.unwrap();
        if content.starts_with(alias) {
            if let Some(i) = content.find(':') {
                let (_, guidStr) = &content.split_at(i + 1);
                println!("Found {guidStr} in {content}");
                match Uuid::parse_str(&guidStr)
                {
                    Ok(id) => return Some(id),
                    Err(e) => return None,
                }
            }
        }
    }
    return None;
}

fn getOrCreateIndex(dataDir: &PathBuf) -> () {
    // Proposal: instead of passing dataDir to other methods,
    // get index and use functions on the index class
}

fn init() -> Result<(), &'static str> {
    // Find git root
    // If at git root - great, proceed
    // Else, show appropriate warning or proceed if a flag is set

    // Init the data folder and the index
    return Ok(());
}

fn switchTask(alias: &str) -> Result<(), &'static str> {
    let dataDirectory = getDataDirectory();
    // find ID in index
    // log
    // switch default to that ID
    // get data of that ID
    // run activation script based on data
    return Ok(());
}

/*
fn getData(id: &Uuid, dataDir: &PathBuf) -> Result<TaskData, &'static str> {
    return Ok(());
}
*/

#[derive(Debug)]
struct TaskData {
    id: uuid::Uuid,
    properties: HashMap<String, String>,
}

