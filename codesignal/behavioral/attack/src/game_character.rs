use crate::attack_strategy::AttackStrategy;

// TODO: Define the GameCharacter struct
// - Create a private field for the character's name
// - Create a private option field for the attack strategy
// - Create a constructor to initialize the character's name
// - Create a method `set_attack_strategy` to set the attack strategy
// - Create a method `perform_attack` to execute the current attack strategy or print a message if no strategy is set
pub struct GameCharacter {
    name: String,
    attack_strategy: Option<Box<dyn AttackStrategy>>,
}

impl GameCharacter {
    pub fn new(name: &str) -> Self {
        GameCharacter {
            name: name.to_string(),
            attack_strategy: None,
        }
    }
    pub fn set_attack_strategy(&mut self, attack_strategy: Box<dyn AttackStrategy>) {
        self.attack_strategy = Some(attack_strategy);
    }
    pub fn perform_attack(&self) {
        if let Some(ref attack_strategy) = self.attack_strategy {
            attack_strategy.execute();
        } else {
            println!("{} attacks with their fists", self.name);
        }
    }
    
    pub fn attack(&mut self) {
        self.perform_attack();
    }
}
