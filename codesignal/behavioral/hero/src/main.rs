mod hero_strategy;

fn main() {
    // TODO: Create an instance of Superhero
    let mut hero = hero_strategy::Superhero::new();

    // TODO: Create instances of:
    // - FlyingStrategy
    // - InvisibleStrategy
    let flying_strategy = Box::new(hero_strategy::FlyingStrategy::new());
    let invisible_strategy = Box::new(hero_strategy::InvisibleStrategy::new());

    // TODO: Set the hero's strategy to FlyingStrategy and call perform_action
    hero.set_strategy(flying_strategy);
    hero.perform_action();

    // TODO: Set the hero's strategy to InvisibleStrategy and call perform_action
    hero.set_strategy(invisible_strategy);
    hero.perform_action();
}
