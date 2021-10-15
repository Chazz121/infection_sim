#[derive(Debug)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug)]
pub struct Person {
  position: Position,
  infected: bool,
}

#[derive(Debug)]
pub struct World {
  map: Vec<Vec<Option<Person>>>
}

impl Person {
  pub fn from(x_pos: usize, y_pos: usize) -> Person {
    Person {
      position: Position { x: x_pos, y: y_pos },
      infected: false,
    }
  }

}

impl World{
  fn create(rows: usize, cols:usize) -> World{
    let columns = vec![Option::None; cols];
    let rows = vec![columns; rows];
    World {
      map: rows
    }
  }

  fn add_person(&mut self, person: Person) {
    let row = 0;
    let col = 0;


    let mut spot = self.map[row][col];

    spot = Option::Some(person)
  }

  pub fn from(rows: usize, cols: usize, population: usize) -> World{
    let mut world = World::create(rows, cols);

    for i in 0..population {
      let person = Person::from(0,0);
      world.add_person(person);
    }

    world
  }
}