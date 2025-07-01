use crate::color::Blue;
use crate::color::Color;
use crate::color::Red;
use crate::shape::Circle;
use crate::shape::Shape;
use crate::shape::Square;

// TODO: Define a trait DrawingFactory
// - Declare an abstract method create_shape()
// - Declare an abstract method create_color()
pub trait DrawingFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
    fn create_color(&self) -> Box<dyn Color>;
}

// TODO: Define a struct GeometricFactory that implements DrawingFactory
// - Implement create_shape() to return a new instance of Square
// - Implement create_color() to return a new instance of Red
pub struct GeometricFactory;

impl DrawingFactory for GeometricFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Square)
    }
    fn create_color(&self) -> Box<dyn Color> {
        Box::new(Red)
    }
}

// TODO: Define a struct ArtisticFactory that implements DrawingFactory
// - Implement create_shape() to return a new instance of Circle
// - Implement create_color() to return a new instance of Blue
pub struct ArtisticFactory;

impl DrawingFactory for ArtisticFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle)
    }
    fn create_color(&self) -> Box<dyn Color> {
        Box::new(Blue)
    }
}
