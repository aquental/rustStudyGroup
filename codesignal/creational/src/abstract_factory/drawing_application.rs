use crate::color::Color;
use crate::drawing_factory::DrawingFactory;
use crate::shape::Shape;

// TODO: Define a struct DrawingApplication
pub struct DrawingApplication {
    shape: Box<dyn Shape>,
    color: Box<dyn Color>,
}

// TODO: Define its fields
// - Private Box<dyn Shape> for shape, initialized with factory
// - Private Box<dyn Color> for color, initialized with factory
// TODO: Define the render() method
// - Invoke the draw() method on the shape field
// - Invoke the fill() method on the color field

impl DrawingApplication {
    pub fn new(factory: Box<dyn DrawingFactory>) -> DrawingApplication {
        DrawingApplication {
            shape: factory.create_shape(),
            color: factory.create_color(),
        }
    }
    pub fn render(&self) {
        self.shape.draw();
        self.color.fill();
    }
}
