# Development

### How to develop
Initial setup
`python -m venv env `

Subsequent setup
`env\Scripts\activate`

### Basic principle
(TODO)
When taskspace is used, the index in user's home directory is updated
The index is a map from task name to location of the .taskfile
So that I can "task switch" to a specific task by name. This would process contents of .taskfile

The .taskfile records all properties pertinent to the task; which files to open, which commands to run; whatever is necessary to get up and running
Some of the properties
- [ ] Open terminal and cd to a path
- [ ] Start a process and open a file
  - [ ] e.g. open IDE in a specific folder and show the readme file
- [ ] Change git branch, or run any git commands
- [ ] Replace file (e.g. the .vs directory with breakpoints, etc)
  - [ ] All artifacts/files are kept in the .taskspace directory
- [ ] Terminal history!!!

Additionally, there are tasks to execute when switching away to another task, such as
- [ ] git stash

Notes:
There might be multiple tasks active in a specific folder
e.g. C:\src\platform has multiple branches, and each task corresponds to specific branch
TODO: how do I design for this?

### Tool usage
task show - show content of the .taskspace
task switch TASKNAME - switch to specific task

## Todo

Crawl up until directory .taskspace is found
If not found, crawl until .gitingore is found and propose initializing taskspace there
Else, propose initialzing taskspace in current directory
Serialize and deserialize tasks in .taskspace/
Store tasks in ~/.taskspace/index


## Useful links
https://simonwillison.net/2023/Sep/30/cli-tools-python/
https://click.palletsprojects.com/en/8.1.x/quickstart/#registering-commands-later
https://github.com/AmadeusW/photobooth/blob/main/sketch.py

# Log

### 2024-10-26
Decided to roll out the Python version, it's error prone but at the same time got MVP up and running in an hour.

### 2024-01 to 2024-02
Initial Rust implementation
