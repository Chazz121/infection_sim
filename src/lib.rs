mod world;
use world::World;

pub fn run() {
    let mut world = World::create(10000, 1000, 400_000, 500);
    /*
    println!("infecting people");
    world.infect_people();
    println!("moving people");
    world.move_people();
    println!("infecting people");
    world.infect_people();
    println!("moving people");
    world.move_people();
    println!("infecting people");
    world.infect_people();
    println!("moving people");
    world.move_people();
    println!("infecting people");
    world.infect_people();
    println!("{:?}", &world);
    */
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
    println!("moving people");
    world.move_people();
}
