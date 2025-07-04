pub trait Weapon {
    // TODO: Define a description method returning "Weapon".
    fn description(&self) -> String;
    // TODO: Define the power method returning a f64.
    fn power(&self) -> f64;
}

#[derive(Clone)]
pub struct Sword;

// TODO: Implement the Weapon trait for Sword and override the power method to return 20.0.
impl Weapon for Sword {
    fn description(&self) -> String {
        String::from("Sword")
    }
    fn power(&self) -> f64 {
        20.0
    }
}

pub struct FirePower {
    decorated_weapon: Box<dyn Weapon>,
}

// TODO: Implement the FirePower struct with a new method and define the Weapon trait for it.
// TODO: Override description method to add " with Fire Power".
// TODO: Override power method to add 10.0 to the weapon's power.
impl FirePower {
    pub fn new(weapon: Box<dyn Weapon>) -> FirePower {
        FirePower {
            decorated_weapon: weapon,
        }
    }
}
impl Weapon for FirePower {
    fn description(&self) -> String {
        let mut desc = self.decorated_weapon.description();
        desc.push_str(" with Fire Power");
        desc
    }
    fn power(&self) -> f64 {
        self.decorated_weapon.power() + 10.0
    }
}

pub struct IcePower {
    decorated_weapon: Box<dyn Weapon>,
}

// TODO: Implement the IcePower struct with a new method and define the Weapon trait for it.
// TODO: Override description method to add " with Ice Power".
// TODO: Override power method to add 8.0 to the weapon's power.
impl IcePower {
    pub fn new(weapon: Box<dyn Weapon>) -> IcePower {
        IcePower {
            decorated_weapon: weapon,
        }
    }
}
impl Weapon for IcePower {
    fn description(&self) -> String {
        let mut desc = self.decorated_weapon.description();
        desc.push_str(" with Ice Power");
        desc
    }
    fn power(&self) -> f64 {
        self.decorated_weapon.power() + 8.0
    }
}
