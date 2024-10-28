import os
import TaskRunner

def get_location_task():
    current_directory = os.getcwd()
    task_path = _get_nearest_task_path(current_directory)
    task_data = get_task_data(task_path)
    return task_data


def switch_task(alias):
    index = _load_index()
    task_path = _get_task_path(alias, index)
    task_data = get_task_data(task_path)
    print(f"Actiavating: {alias}")
    TaskRunner.execute_task(task_data, task_path)


def get_task_data(data_path):
    task_map = {}
    if os.path.exists(data_path):
        with open(data_path, "r") as file:
            for line in file:
                key, value = line.strip().split("=", 1)
                task_map[key] = value
    return task_map


def print_task(task_data):
    if not task_data:
        print("No task data available")
        return

    print("\nTask properties:")
    for key, value in task_data.items():
        print(f"  {key}: {value}")


def _load_index():
    home_dir = os.path.expanduser("~")
    index_file_path = os.path.join(home_dir, ".taskspace", "index")
    index_map = {}
    if os.path.exists(index_file_path):
        with open(index_file_path, "r") as file:
            for line in file:
                # Split each line into key and value parts
                key, value = line.strip().split("=", 1)
                index_map[key] = value
    else:
        with open(index_file_path, "w") as file:
            file.write("")
    return index_map


def _update_index(new_index):
    home_dir = os.path.expanduser("~")
    index_file_path = os.path.join(home_dir, ".taskspace", "index")
    if os.path.exists(index_file_path):
        with open(index_file_path, "w") as file:
            for key, value in new_index:
                file.write(f"{key}={value}\n")


def _get_task_path(alias, index):
    return index[alias]


def _get_nearest_task_path(path):
    """Find the nearest .taskspace directory by traversing up the directory tree.

    Args:
        current_directory: The starting directory path to search from

    Returns:
        str: Path to the nearest .taskspace directory, or None if not found
    """
    current_path = os.path.abspath(path)

    while current_path != os.path.sep:
        candidate_path = os.path.join(current_path, ".taskspace")
        if os.path.exists(candidate_path):
            return candidate_path
        current_path = os.path.dirname(current_path)
    return None
