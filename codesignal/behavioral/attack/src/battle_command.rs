use crate::game_character::GameCharacter;


// TODO: Define the BattleCommand struct implementing Command
// - Create a private field for the character
// - Create a constructor to initialize the character
// - Implement the execute method to perform the character's attack
pub struct BattleCommand {
    character: GameCharacter,
}

impl BattleCommand {
    pub fn new(character: GameCharacter) -> BattleCommand {
        BattleCommand { character }
    }
    
    pub fn execute(&mut self) {
        self.character.attack();
    }
}
