mod zoo;

use zoo::{Animal, AnimalComponent, Zoo};

fn main() {
    // TODO: Create the main Zoo instance
    let mut main_zoo = Zoo::new();
    main_zoo.add_animal(Box::new(Animal::new("Leo", "Lion")));
    main_zoo.add_animal(Box::new(Animal::new("Tina", "Tiger")));
    main_zoo.add_animal(Box::new(Animal::new("Benny", "Bear")));

    // TODO: Create a sub-zoo for big cats
    let mut sub_zoo = Zoo::new();

    // TODO: Add the lion and tiger to the sub-zoo
    sub_zoo.add_animal(Box::new(Animal::new("Leo", "Lion")));
    sub_zoo.add_animal(Box::new(Animal::new("Tina", "Tiger")));

    // TODO: Add the bear to the main zoo
    main_zoo.add_animal(Box::new(Animal::new("Benny", "Bear")));

    // TODO: Add the sub-zoo to the main zoo
    main_zoo.add_animal(Box::new(sub_zoo));

    // TODO: Display details of all zoo components in the main zoo
    main_zoo.display_details();
}
