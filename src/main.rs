use infection_sim::World;

fn main() {

  let mut world = World::create(1, 1, 2, 1);
  world.infect_people();
  println!("{:?}", &world);

  world.move_people();
  world.infect_people();
  println!("{:?}", &world);
  world.move_people();
  world.infect_people();
  println!("{:?}", &world);
  world.move_people();
  world.infect_people();
  println!("{:?}", &world);
}
