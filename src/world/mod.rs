use std::collections::HashMap;

mod person;
use person::{Person, Boundry};

#[derive(Debug)]
pub struct World {
  people: Vec<Person>,
  boundry: Boundry,
}

impl World {
  pub fn create(
    height: u64,
    width: u64,
    population: u64,
    mut num_of_infected: u64,
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
    let mut infected_status_and_position = HashMap::new();

    for person in &self.people {
      let status_bucket = infected_status_and_position.entry(*person.is_infected()).or_insert(Vec::new());
      status_bucket.push(person.get_position())
    }

    let infected_bucket = infected_status_and_position.entry(true).or_insert(Vec::new());

    for person in &mut self.people {
      if infected_bucket.contains(&person.get_position()) {
        person.infect()
      }
    }
  }
}
