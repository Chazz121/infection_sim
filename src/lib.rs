use std::collections::HashMap;

#[derive(Debug)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug)]
struct Boundry {
  height: usize,
  width: usize,
}

#[derive(Debug)]
struct Person {
  position: Position,
  infected: bool,
}

#[derive(Debug)]
pub struct World {
  people: Vec<Person>,
  boundry: Boundry,
}
impl std::cmp::PartialEq for Position{
  fn eq(&self, other: &Position) -> bool{
    self.x == other.x && self.y == other.y
  }
}

impl Clone for Position{
  fn clone(&self) -> Self {
    Position{
      x: self.x,
      y: self.y
    }
  }
}

impl Person {
  pub fn from(x_pos: usize, y_pos: usize, boundry: &Boundry, infected: bool) -> Person {
    Person {
      position: Position { x: x_pos, y: y_pos },
      infected,
    }
  }

  pub fn random(boundry: &Boundry, infected: bool) -> Person {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let y_pos = rng.gen_range(0..=boundry.height);
    let x_pos = rng.gen_range(0..=boundry.width);

    let position = Position { x: x_pos, y: y_pos };

    Person {
      infected,
      position: position,
    }
  }

  pub fn walk(&mut self, boundry: &Boundry) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    if boundry.width == 0 {
      ();
    } else if self.position.x == 0 {
      self.position.x = self.position.x + rng.gen_range(0..=1)
    } else if self.position.x == boundry.width {
      self.position.x = self.position.x - rng.gen_range(0..=1)
    } else {
      let operation = rng.gen_range(0..1);
      if operation == 0 {
        self.position.x = self.position.x + rng.gen_range(0..=1)
      } else {
        self.position.x = self.position.x - rng.gen_range(0..=1)
      }
    }

    if boundry.height == 0 {
      ();
    } else if self.position.y == 0 {
      self.position.y = self.position.y + rng.gen_range(0..=1)
    } else if self.position.y == boundry.width {
      self.position.y = self.position.y - rng.gen_range(0..=1)
    } else {
      let operation = rng.gen_range(0..1);
      if operation == 0 {
        self.position.y = self.position.y + rng.gen_range(0..=1)
      } else {
        self.position.y = self.position.y - rng.gen_range(0..=1)
      }
    }
  }

  pub fn infect(&mut self) {
    self.infected = true
  }
}

impl World {

  pub fn create(
    height: usize,
    width: usize,
    population: usize,
    mut num_of_infected: usize,
  ) -> World {
    let boundry = Boundry { height, width };

    let mut people: Vec<Person> = Vec::new();

    for _ in 0..population {
      if num_of_infected > 0 {
        let person = Person::random(&boundry, true);
        people.push(person);
        num_of_infected -= 1;
      } else {
        let person = Person::random(&boundry, false);
        people.push(person)
      }
    }

    World { people, boundry }
  }

  pub fn move_people(&mut self) {
    for person in &mut self.people {
      person.walk(&self.boundry)
    }
  }

  pub fn infect_people(&mut self) {
    let mut infection_list: Vec<Position> = Vec::new();

    for person in &mut self.people {
      if person.infected {
        infection_list.push(person.position.clone())
      }
    }

    for person in &mut self.people {
      if infection_list.contains(&mut person.position) {
          person.infect()
      }
    }
  }
}
