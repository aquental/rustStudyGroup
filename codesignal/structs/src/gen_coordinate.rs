struct Coordinate<T, U> {
    latitude: T,
    longitude: U,
}

impl<T, U> Coordinate<T, U> {
    fn new(latitude: T, longitude: U) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    fn latitude(&self) -> &T {
        &self.latitude
    }

    fn longitude(&self) -> &U {
        &self.longitude
    }
}

fn main() {
    let location = Coordinate::new(51.5074, "North");
    println!("Latitude: {}", location.latitude());
    println!("Longitude: {}", location.longitude());
}
