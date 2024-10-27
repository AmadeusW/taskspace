import os
import TaskRunner

def switch_task(alias):
    index = load_index()
    task_path = get_task_path(alias, index)
    task_data = get_task_data(task_path)
    print(f"Actiavating: {alias}")
    TaskRunner.execute_task(task_data, task_path)

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

