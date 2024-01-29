import uuid

class Task:
    def __init__(self):
        guid = uuid.uuid4() 
        self.properties = {"id": guid}
    
    def Show(self):
        print("Model dump:")
        for key, value in self.properties.items():
            print(f"Key {key}, Value {value.GetValue()}")

    def SetValue(self, key, value)
        self.properties[key] = value
   
    def __getitem__(self, key):
        return self.properties[key]
