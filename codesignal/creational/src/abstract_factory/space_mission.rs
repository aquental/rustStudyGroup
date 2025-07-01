use crate::space_factory::SpaceFactory;
use crate::spaceship::Spaceship;
use crate::spacesuit::SpaceSuit;

// TODO: Define a struct `SpaceMission` that takes a Box<dyn SpaceFactory> parameter.

// TODO: Define private fields for `spaceship` and `space_suit`, initialized using the factory.

// TODO: Define the `prepare()` method to perform the following:
// - Call `blast_off()` on the `spaceship`.
// - Call `wear()` on the `space_suit`.

pub struct SpaceMission {
    spaceship: Box<dyn Spaceship>,
    space_suit: Box<dyn SpaceSuit>,
}

impl SpaceMission {
    pub fn new(factory: Box<dyn SpaceFactory>) -> Self {
        SpaceMission {
            spaceship: factory.create_spaceship(),
            space_suit: factory.create_space_suit(),
        }
    }

    pub fn prepare(&self) {
        self.spaceship.blast_off();
        self.space_suit.wear();
    }
}
