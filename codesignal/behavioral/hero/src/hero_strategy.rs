pub trait HeroStrategy {
    // TODO: Define a method named execute
    fn execute(&self);
}

pub struct FlyingStrategy;
impl FlyingStrategy {
    pub fn new() -> Self {
        Self
    }
}
// TODO: Implement HeroStrategy for FlyingStrategy
//   - Implement the execute method to output: "Hero is flying!"
impl HeroStrategy for FlyingStrategy {
    fn execute(&self) {
        println!("Hero is flying!");
    }
}
pub struct InvisibleStrategy;
impl InvisibleStrategy {
    pub fn new() -> Self {
        Self
    }
}
// TODO: Implement HeroStrategy for InvisibleStrategy
//   - Implement the execute method to output: "Hero is invisible!"
impl HeroStrategy for InvisibleStrategy {
    fn execute(&self) {
        println!("Hero is invisible!");
    }
}

pub struct Superhero {
    strategy: Option<Box<dyn HeroStrategy>>,
}

impl Superhero {
    // TODO: Implement a constructor function `new` that initializes the strategy to None
    pub fn new() -> Self {
        Self { strategy: None }
    }

    // TODO: Define a method set_strategy to set the hero's strategy
    pub fn set_strategy(&mut self, strategy: Box<dyn HeroStrategy>) {
        self.strategy = Some(strategy);
    }

    // TODO: Define a method perform_action to:
    //   - Execute the current strategy's execute method
    //   - Or print "No strategy set." if no strategy is set
    pub fn perform_action(&self) {
        if let Some(strategy) = &self.strategy {
            strategy.execute();
        } else {
            println!("No strategy set.");
        }
    }
}
