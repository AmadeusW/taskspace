import subprocess

class GitAdapter:
    def switch(self, branch):
        subprocess.call(["git", "checkout", f"{branch}"])
