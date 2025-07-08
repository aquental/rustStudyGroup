use crate::game_character::GameCharacter;

// TODO: Define a trait Command with a method execute
pub trait Command {
    fn execute(&self, character: &mut GameCharacter);
}

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
}
impl Command for BattleCommand {
    fn execute(&self, character: &mut GameCharacter) {
        character.perform_attack();
    }
}
