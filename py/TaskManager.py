import os
import subprocess

def switch_task(alias):
    index = load_index()
    task_path = get_task_path(alias, index)
    task_data = get_task_data(task_path)
    print(f"Actiavating: {alias}")
    execute_task(task_data, task_path)

def load_index():
    home_dir = os.path.expanduser("~")
    index_file_path = os.path.join(home_dir, ".taskspace", "index")
    index_map = {}
    if os.path.exists(index_file_path):
        with open(index_file_path, "r") as file:
            for line in file:
                # Split each line into key and value parts
                key, value = line.strip().split("=", 1)
                index_map[key] = value
    return index_map

def get_task_path(alias, index):
    return index[alias]

def get_task_data(task_path):
    task_map = {}
    data_path = os.path.join(task_path, ".taskspace")
    if os.path.exists(data_path):
        with open(data_path, "r") as file:
            for line in file:
                # Split each line into key and value parts
                key, value = line.strip().split("=", 1)
                task_map[key] = value
    return task_map

def execute_task(task_data, root_path):
    try:
        print(f"Change directory: {root_path}")
        subprocess.run(["start", "", f"cd {root_path}"], shell=True, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to change directory: {e}")

    if "readme" in task_data:
        readme_path = os.path.join(root_path, task_data["readme"])
        try:
            print(f"Opening file: {readme_path}")
            subprocess.run(["start", "", readme_path], shell=True, check=True)
        except subprocess.CalledProcessError as e:
            print(f"Failed to open file: {e}")
    else:
        print("The 'readme' key does not exist in the task data.")

    if "url" in task_data:
        task_url = task_data["url"]
        try:
            print(f"Opening link: {task_url}")
            subprocess.run(["start", "", task_url], shell=True, check=True)
        except subprocess.CalledProcessError as e:
            print(f"Failed to open link: {e}")
    else:
        print("The 'url' key does not exist in the task data.")