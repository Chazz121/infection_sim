use infection_sim::Person;
use infection_sim::World;

fn main() {
  let person = Person::from(35,2);

  println!("person: {:#?}", person);

  let world = World::from(1, 1, 1);

  println!("{:#?}", world);
}
