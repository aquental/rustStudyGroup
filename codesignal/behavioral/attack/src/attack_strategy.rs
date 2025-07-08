// TODO: Define a trait AttackStrategy with a method execute
pub trait AttackStrategy {
    fn execute(&self);
}

// TODO: Define two concrete strategy structs implementing AttackStrategy:
// - SwordAttack
// - BowAttack
pub struct SwordAttack;
pub struct BowAttack;

// TODO: Implement the execute method in SwordAttack to print "Performs a sword attack!"
impl AttackStrategy for SwordAttack {
    fn execute(&self) {
        println!("Performs a sword attack!");
    }
}

// TODO: Implement the execute method in BowAttack to print "Performs a bow attack!"
impl AttackStrategy for BowAttack {
    fn execute(&self) {
        println!("Performs a bow attack!");
    }
}
