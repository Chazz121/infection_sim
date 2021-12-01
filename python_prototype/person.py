
class Person:
    def __init__(self) -> None:
        self.infected = False

    def isInfected(self) -> bool:
        return self.infected
    
    def infect(self):
        self.infected = True
