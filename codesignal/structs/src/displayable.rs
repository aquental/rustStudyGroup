/*
Implement Displayable for the Car struct, which requires a method to display information.
Use a trait object to display the car's information dynamically.
*/
trait Displayable {
    fn display(&self);
}

struct Car {
    make: String,
    model: String,
}

// TODO: Implement the Displayable trait for the Car struct
impl Displayable for Car {
    fn display(&self) {
        println!("Car: {} {}", self.make, self.model);
    }
}

// TODO: Implement a show_info functions that accepts a Displayable (via trait objects) and calls display()
fn show_info(item: &dyn Displayable) {
    item.display();
}

fn main() {
    let car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
    };

    // TODO: Use the show_info function to display car information
    show_info(&car);
}
