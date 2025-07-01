use crate::color::Color;
use crate::drawing_factory::DrawingFactory;
use crate::shape::{Circle, Rectangle, Shape, Square};

// TODO: Define a struct DrawingApplication
struct DrawingApplication {
    shape_factory: DrawingFactory,
    color_factory: DrawingFactory,
}

// TODO: Define its fields
// - Private Box<dyn Shape> for shape, initialized with factory
// - Private Box<dyn Color> for color, initialized with factory
// TODO: Define the render() method
// - Invoke the draw() method on the shape field
// - Invoke the fill() method on the color field

impl DrawingApplication {
    fn new() -> DrawingApplication {
        DrawingApplication {
            shape_factory: DrawingFactory::new(),
            color_factory: DrawingFactory::new(),
        }
    }
    fn render(&self) {
        self.shape_factory.draw();
        self.color_factory.fill();
    }
}
