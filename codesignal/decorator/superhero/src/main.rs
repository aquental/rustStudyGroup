mod weapon;
use weapon::{FirePower, IcePower, Sword, Weapon};

fn main() {
    // TODO: Create a new Sword object and print its description and power.
    let sword = Sword {};

    // TODO: Wrap the sword object with FirePower and print the updated description and power.
    let fire_sword = FirePower::new(Box::new(sword.clone()));
    println!("Fire Sword: {}", fire_sword.description());
    println!("Fire Sword Power: {}", fire_sword.power());

    // TODO: Wrap the sword object with IcePower and print the updated description and power.
    let ice_sword = IcePower::new(Box::new(sword.clone()));
    println!("Ice Sword: {}", ice_sword.description());
    println!("Ice Sword Power: {}", ice_sword.power());

    let fire_ice_sword = FirePower::new(Box::new(ice_sword));
    println!("Fire Ice Sword: {}", fire_ice_sword.description());
    println!("Fire Ice Sword Power: {}", fire_ice_sword.power());
}
