#[derive(Debug)]
struct Weapon {
    name: String,
    power: Powers,
}
#[derive(Debug)]
enum Powers {
    easy,
    med,
    hard,
    ultra,
}

impl Weapon {
    fn default() -> Self {
        Self {
            name: "knife".to_owned(),
            power: Powers::easy,
        }
    }

    fn check(&self) -> (&String, &Powers) {
        (&self.name, &self.power)
    }

    fn upgrade(&mut self) {
        self.name = "Gun".to_owned();
        self.power = Powers::hard;
    }

    fn know_your_weapon(&self) {
        match &self.power {
            Powers::easy => println!(" this is bigginer level {:?}", Powers::easy),
            Powers::med => println!(" this is med level {:?}", Powers::med),
            Powers::hard => println!(" this is hard level {:?}", Powers::hard),
            Powers::ultra => println!(" this is ultra level {:?}", Powers::ultra),
        }
    }
}

fn main() {
    let mut u1 = Weapon::default();

    println!("{:?}", u1.know_your_weapon());
    u1.upgrade();
    println!("{:?}", u1.know_your_weapon());
}
