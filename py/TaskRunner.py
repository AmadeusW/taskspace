import os
import subprocess

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

    if "branch" in task_data:
        branch = task_data["branch"]
        try:
            # TODO: try stashing. bail if unable to stash (e.g. due to rebase/merge in progress)
            print(f"Switching branch: {branch}")
            subprocess.call(["git", "checkout", f"{branch}"])
        except subprocess.CalledProcessError as e:
            print(f"Failed to switch branch: {e}")
    else:
        print("The 'branch' key does not exist in the task data.")
