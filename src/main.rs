use std::fs;
use std::io::{stdin, BufRead, BufReader, Write};
use uuid::Uuid;
use std::path::PathBuf;
use std::collections::HashMap;

const DATADIR: &str = ".taskspace";
const GITDIR: &str = ".git";
const INDEXFILE: &str = "index.txt";
const CURRENTFILE: &str = "current.txt";

fn main() {
    println!("Hello, world!");
    let mut dir = get_data_directory();
    if dir.is_none() {
        println!("Did not find data directory.");
        let init_dir = init();
        if init_dir.is_none() {
            println!("Exiting.");
            return;
        } else {
            dir = init_dir;
        }
    }
    let data_dir = dir.unwrap();
    println!("Found data directory {}", data_dir.display());

    let current_alias = get_current_alias(&data_dir)
        .expect("Expected to find current alias"); // TODO: Handle cases when there is no alias, e.g. new repo
    let current_id = get_id_from_index(&current_alias, &data_dir)
        .expect("Expected to find ID of the alias");
    
    if let task_data = get_task_data(&current_id, &data_dir) {
        println!("Current task: {}", current_alias);
    }
    
}

fn get_task_data(id: &uuid::Uuid, data_dir: &PathBuf) -> Option<TaskData> {
    let task_data = TaskData {
        id: Uuid::from_u128(0x0u128),
        sample: String::from("sample text")
    };
    Some(task_data)
}

fn get_data_directory() -> Option<PathBuf> {
    let current_dir = std::env::current_dir()
        .expect("Expected to find the current directory");
    return find_path_in_tree(&current_dir, DATADIR);
}

fn get_git_directory() -> Option<PathBuf> {
    let current_dir = std::env::current_dir()
        .expect("Expected to find the current directory");
    return find_path_in_tree(&current_dir, GITDIR);
}

fn find_path_in_tree(start_dir: &PathBuf, target_dir: &str) -> Option<PathBuf> {
    let mut current_path = start_dir.clone();
    loop {
        // Check if the target path is found
        let candidate_path = current_path.join(target_dir);
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
    fs::create_dir_all(&task_path)
        .expect(&format!("Expected to be able to create {:?}", &task_path));

    return append_to_index(alias, &id, data_dir);
}

fn append_to_index(alias: &str, id: &uuid::Uuid, data_dir: &PathBuf) -> Result<(), &'static str> {
    let index_path = data_dir.join(INDEXFILE);
    let mut file = fs::File::create(&index_path)
        .expect(&format!("Expected to access the index file {}", index_path.display()));
    writeln!(&mut file, "{alias}:{id}");
    return Ok(());
}

fn get_current_alias(data_dir: &PathBuf) -> Option<String> {
    let target_path = data_dir.join(CURRENTFILE);
    let f = match fs::File::open(&target_path) {
        Ok(file) => file,
        Err(e) => return None,
    };

    let reader = BufReader::new(f);
    let mut content = String::new();
    for line in reader.lines() {
        // we are interested in just the single line
        match line {
            Ok(x) => return Some(x),
            Err(x) => return None,
        }
    }
    None
}

fn get_id_from_index(alias: &str, data_dir: &PathBuf) -> Option<Uuid> {// Result<Uuid, String> {
    let index_path = data_dir.join(INDEXFILE);
    let f = fs::File::open(&index_path)
        .expect(&format!("Expected to open the index file {}", index_path.display()));
    let reader = BufReader::new(f);
    
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

fn create_index(target_dir: &PathBuf) -> Option<PathBuf> {
    // Proposal: instead of passing dataDir to other methods,
    // get index and use functions on the index class
    println!("Creating index: {:?}", target_dir);
    let candidate_path = target_dir.join(DATADIR);
    fs::create_dir(candidate_path.as_path())
        .expect("Expected to create directory"); // todo: better error message and return value
    Some(candidate_path)
}

fn init() -> Option<PathBuf> {
    let git_dir_candidate = get_git_directory();
    let mut input = String::new();
    if (git_dir_candidate.is_some()) {
        let mut git_dir = git_dir_candidate.unwrap();
        println!("Found git directory: {:?}", git_dir);
        git_dir.pop();
        println!("Found candidate directory: {:?}", git_dir);
        println!("Do you want to initialize taskspace in the candidate directory? [y][n]");
        stdin().lock().read_line(&mut input)
            .expect("Expected to receive user input");
        match input.trim_end().to_lowercase().as_str() {
            "y" => {
                create_index(&git_dir);
                return Some(git_dir);
            }
            _ => ()
        }
    }

    let current_dir = std::env::current_dir()
        .expect("Expected to find the current directory");
    println!("Current directory: {:?}", current_dir);
    println!("Do you want to initialize taskspace in current directory? [y][n]");
    stdin().lock().read_line(&mut input)
        .expect("Expected to receive user input");
    match input.trim_end().to_lowercase().as_str() {
        "y" => {
            create_index(&current_dir);
            return Some(current_dir);
        }
        _ => ()
    }

    None
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
    id: Uuid,
    //properties: HashMap<String, String>,
    sample: String,
}

