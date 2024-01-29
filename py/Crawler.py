import os

DATA_FOLDER = ".taskspace"
INDEX_FILE = "index.txt"

def FindDataFolder(path):
    return FindFolder(path, DATA_FOLDER)

def FindFolder(path, target):
    currentPath = os.path.abspath(path)

    while currentPath != os.path.sep:
        if target in os.listdir(currentPath):
            return os.path.join(currentPath, target)
        currentPath = os.path.dirname(currentPath)

    return None

def FindIdInIndex(path, alias):
    indexPath = os.path.join(path, INDEX_FILE)
    if alias is None:
        alias = "default"
    line = FindInIndex(indexPath, alias)


def FindInIndex(filename, target):
    with open(filename, 'r') as file:
        for line in file:
            if target in line:
                # Match found, you can process the line or perform other actions
                print("Match found in line:", line.strip())
                return line.strip()
                break
    return None


def AddToIndex(path, id, alias):
    indexPath = os.path.join(path, INDEX_FILE)

def SetIndexDefault(path, id):
    

def GetData(alias):
    # alias is optional
    currentDirectory = os.getcwd()
    dataFolder = FindDataFolder(currentDirectory)
    
    if dataFolder is None:
        print "Unable to find taskspace data"
        return None

    id = FindInIndex(dataFolder, alias)
    if id is None:
        print f"Unable to find {alias} in the index"
    
    print f"Alias {alias} has ID {id}"
