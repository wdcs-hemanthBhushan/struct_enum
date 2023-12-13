#[derive(Debug)]
struct Weapon {
    name: String,
    power: Powers,
    strenth: i32,
}
#[derive(Debug)]
enum Powers {
    Easy,
    Med,
    Hard,
    Ultra,
}

impl Weapon {
    fn default() -> Self {
        Self {
            name: "knife".to_owned(),
            power: Powers::Easy,
            strenth: 23,
        }
    }

    fn check_strnth(&mut self) -> &i32 {
        self.strenth += 26;
        &self.strenth
    }

    fn claim_weapon(&self) {
        match &self.strenth {
            0..=23 => println!("stil in the biggner level"),
            24..=100 => println!("your eligible to claim the Ultra weapon"),
            _ => println!("your powerfullll"),
        }
    }

    fn upgrade(&mut self) {
        match &self.strenth {
            24..=100 => {
                self.name = "Gun".to_owned();
                self.power = Powers::Ultra;
            }
            _ => println!("ur not eligible to upgrade"),
        }
    }

    fn know_your_weapon(&self) {
        match &self.power {
            Powers::Easy => println!(" this is bigginer level {:?}", Powers::Easy),
            Powers::Med => println!(" this is Med level {:?}", Powers::Med),
            Powers::Hard => println!(" this is Hard level {:?}", Powers::Hard),
            Powers::Ultra => println!(" this is Ultra level {:?}", Powers::Ultra),
        }
    }
}

fn main() {
    let mut u1 = Weapon::default();

    u1.know_your_weapon();

    u1.check_strnth();
    u1.claim_weapon();
    u1.upgrade();
    u1.know_your_weapon();
    u1.check_strnth();
    u1.claim_weapon();
    u1.know_your_weapon();
}
