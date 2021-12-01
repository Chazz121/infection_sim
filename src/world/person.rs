
#[derive(Debug)]
pub struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug)]
pub struct Boundry {
    pub height: u64,
    pub width: u64,
}

#[derive(Debug)]
pub struct Person {
    position: Position,
    infected: bool,
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl Person {
    /*
    pub fn from(x_pos: u64, y_pos: u64, boundry: &Boundry, infected: bool) -> Person {
        Person {
            position: Position { x: x_pos, y: y_pos },
            infected,
        }
    }
    */
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

    pub fn is_infected(&self) -> &bool {
        &self.infected
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
