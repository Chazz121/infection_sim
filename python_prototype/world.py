
from person import Person
from random import randrange

class Boundry:
    def __init__(self, height, width):
        self.height = height
        self.width = width
    
    def isWithin(self, x, y):
        if ((x<self.width) and (y<self.height)):
            return True
        return False
    
    def getRandPosWithin(self):
        x = randrange(self.width)
        y = randrange(self.height)

        return x,y
class World:
    def __init__(self, population_size, boundry = Boundry(10,10)) -> None:
        self.boundry = boundry
        self.population = [(Person(), self.boundry.getRandPosWithin())for _ in range(population_size)]


w = World(4_000, Boundry(10,10))

print(w.population)