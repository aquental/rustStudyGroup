/*
Troubleshoot code (./src/car.rs) involving a Car struct that tries to update its speed. The code contains a bug that prevents the speed from changing correctly.
Find and fix the issue to make the code work properly.
*/
struct Car {
    speed: i32,
}

impl Car {
    // Associated function (constructor)
    fn new(speed: i32) -> Self {
        Self { speed }
    }

    // Method to update car speed
    fn update_speed(&mut self, new_speed: i32) {
        self.speed = new_speed;
    }

    // Method to get car speed
    fn get_speed(self) -> i32 {
        self.speed
    }
}

fn main() {
    let mut my_car = Car::new(60);
    my_car.update_speed(80);
    println!("Updated car speed: {}", my_car.get_speed());
}
