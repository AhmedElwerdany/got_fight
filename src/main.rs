use rand::Rng;
use std::cmp;

enum Spell {
    BURN,
    LOSE_IN_WAR,
}

enum Swords {
    BIG,
    FIRE,
    ICE,
    SPELL(Spell),
}

struct Fight {
    character: Character,
    other_character: Character,
}

impl Fight {
    fn fight(&mut self) {
        while self.character.health > 0 && self.other_character.health > 0 {
            Fight::random_hit(&mut self.character, &mut self.other_character);
        }

        if self.character.is_dead() {
            println!("\n{} is dead", self.character.name);
        } else if self.other_character.is_dead() {
            println!("\n{} is dead", self.other_character.name);
        };
    }

    fn random_hit(character: &mut Character, other_character: &mut Character) -> i32 {
        let random = rand::thread_rng().gen_range(0..=1);
        if random == 0 {
            println!("> {} is about to hit", character.name);
            return character.hit(other_character);
        } else {
            println!("> {} is about to hit", other_character.name);
            return other_character.hit(character);
        }
    }

   

    
}



struct Character {
    name: String,
    sword: Swords,
    age: u8,
    health: i32,
}

impl Character {
    fn can_fight(&self) -> bool {
        match &self.age.cmp(&(20)) {
            cmp::Ordering::Less => false,
            cmp::Ordering::Equal => false,
            cmp::Ordering::Greater => true,
        }
    }

    fn hit(&self, other: &mut Character) -> i32 {
        let damage = match &self.sword {
            Swords::BIG => 20,
            Swords::FIRE => 50,
            Swords::ICE => 70,
            Swords::SPELL(spell) => match spell {
                Spell::BURN => 90,
                Spell::LOSE_IN_WAR => 100,
            },
        };

        other.health = match (other.health - damage).cmp(&(0)) {
            cmp::Ordering::Less => 0,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => other.health - damage,
        };
        damage
    }

    fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

fn main() {
    let mut john_snow = Character {
        name: String::from("John Snow"),
        sword: Swords::FIRE,
        age: 29,
        health: 100,
    };

    let mut red_witch = Character {
        name: String::from("Red Witch"),
        sword: Swords::SPELL(Spell::BURN),
        age: 30,
        health: 100,
    };

    let mut fight  = Fight {
        character: john_snow,
        other_character: red_witch,
    };

    fight.fight();
   
}
