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

fn get_data_directory() -> Option<PathBuf> {
    let current_dir = std::env::current_dir()
        .expect("Expected to find the current directory");
    return find_path_in_tree(&current_dir, DATADIR);
}

fn find_path_in_tree(start_dir: &PathBuf, target_dir: &str) -> Option<PathBuf> {
    loop {
        let mut current_path = start_dir.clone();
        // Check if the target path is found
        let candidate_path = current_path.join(DATADIR);
        if candidate_path.exists() {
            return Some(candidate_path);
        }

        // Move up to the parent directory
        if !current_path.pop() {
            // If we can't go up further, break the loop
            break;
        }
    }

    // If the target path is not found in the entire directory tree
    None
}

fn create_task(alias: &str, data_dir: &PathBuf) -> Result<(), &'static str> {
    let id = Uuid::new_v4();

    let task_path = data_dir.join(format!("{}", id));
    fs::create_dir_all(&task_path);

    return append_to_index(alias, &id, data_dir);
}

fn append_to_index(alias: &str, id: &uuid::Uuid, data_dir: &PathBuf) -> Result<(), &'static str> {
    let index_path = data_dir.join(INDEXFILE);
    let mut file = fs::File::create(&index_path)
        .expect(&format!("Expected to access the index file {}", index_path.display()));
    writeln!(&mut file, "{alias}:{id}");
    return Ok(());
}

fn get_id_from_index(alias: &str, data_dir: &PathBuf) -> Option<Uuid> {// Result<Uuid, String> {
    let index_path = data_dir.join(INDEXFILE);
    let file = fs::File::open(&index_path)
        .expect(&format!("Expected to open the index file {}", index_path.display()));
    let reader = BufReader::new(file);
    
    // Iterate over lines in the file
    for line in reader.lines() {
        let content = line.unwrap();
        if content.starts_with(alias) {
            if let Some(i) = content.find(':') {
                let (_, guid_str) = &content.split_at(i + 1);
                println!("Found {guid_str} in {content}");
                match Uuid::parse_str(&guid_str)
                {
                    Ok(id) => return Some(id),
                    Err(e) => return None,
                }
            }
        }
    }
    return None;
}

fn get_or_create_index(data_dir: &PathBuf) -> () {
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

fn switch_task(alias: &str) -> Result<(), &'static str> {
    let data_directory = get_data_directory();
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

