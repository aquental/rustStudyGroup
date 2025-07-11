// Define the Component trait for shapes and composites
trait Component {
    fn draw(&self);
}

// Leaf: Circle shape
struct Circle {
    name: String,
}

impl Circle {
    fn new(name: &str) -> Self {
        Circle {
            name: name.to_string(),
        }
    }
}

impl Component for Circle {
    fn draw(&self) {
        println!("Drawing Circle: {}", self.name);
    }
}

// Leaf: Square shape
struct Square {
    name: String,
}

impl Square {
    fn new(name: &str) -> Self {
        Square {
            name: name.to_string(),
        }
    }
}

impl Component for Square {
    fn draw(&self) {
        println!("Drawing Square: {}", self.name);
    }
}

// Composite: NestedWindow that holds multiple shapes
struct NestedWindow {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl NestedWindow {
    fn new(name: &str) -> Self {
        NestedWindow {
            name: name.to_string(),
            components: Vec::new(),
        }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}

impl Component for NestedWindow {
    fn draw(&self) {
        println!("Drawing NestedWindow: {}", self.name);
        for component in &self.components {
            component.draw();
        }
    }
}

fn main() {
    // Create individual shapes
    let circle1 = Circle::new("Circle1");
    let circle2 = Circle::new("Circle2");
    let square1 = Square::new("Square1");

    // Create a nested window and add shapes to it
    let mut window = NestedWindow::new("MainWindow");
    window.add(Box::new(circle1));
    window.add(Box::new(circle2));
    window.add(Box::new(square1));

    // Draw all shapes in the window
    window.draw();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_draw() {
        let circle = Circle::new("TestCircle");
        circle.draw(); // Should print to stdout
    }

    #[test]
    fn test_square_draw() {
        let square = Square::new("TestSquare");
        square.draw(); // Should print to stdout
    }

    #[test]
    fn test_nested_window_draw() {
        let mut window = NestedWindow::new("TestWindow");
        window.add(Box::new(Circle::new("TestCircle")));
        window.add(Box::new(Square::new("TestSquare")));
        window.draw(); // Should print window and shapes to stdout
    }

    #[test]
    fn test_empty_nested_window() {
        let window = NestedWindow::new("EmptyWindow");
        window.draw(); // Should print only the window name to stdout
    }
}
