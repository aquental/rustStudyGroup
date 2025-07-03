use crate::employee::Employee;

pub struct Developer {
    name: String,
    position: String,
}

impl Developer {
    pub fn new(name: &str, position: &str) -> Self {
        Developer {
            name: name.to_string(),
            position: position.to_string(),
        }
    }
}

impl Employee for Developer {
    fn display(&self, depth: usize) {
        println!(
            "{}- {} works as {}.",
            "  ".repeat(depth),
            self.name,
            self.position
        );
    }
}
