mod attack_strategy;
mod battle_command;
mod game_character;

use attack_strategy::{BowAttack, SwordAttack};
use battle_command::BattleCommand;
use game_character::GameCharacter;

fn main() {
    // TODO: Create two GameCharacter instances with names "Knight" and "Archer"
    let mut knight = GameCharacter::new("Knight");
    let mut archer = GameCharacter::new("Archer");

    // TODO: Create two strategy instances: SwordAttack and BowAttack
    let sword_attack = SwordAttack {};
    let bow_attack = BowAttack {};

    // TODO: Set the attack strategies for the characters
    knight.set_attack_strategy(Box::new(sword_attack));
    archer.set_attack_strategy(Box::new(bow_attack));

    // TODO: Create two BattleCommand instances for the characters
    let mut knight_command = BattleCommand::new(knight);
    let mut archer_command = BattleCommand::new(archer);

    // TODO: Execute the battle commands
    knight_command.execute();
    archer_command.execute();
}
