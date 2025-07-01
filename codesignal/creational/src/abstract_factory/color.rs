// TODO: Define a trait Color
// - Declare an abstract method fill()
pub trait Color {
    fn fill(&self);
}

// TODO: Define a struct Red that implements Color
// - Implement the fill() method to print "Filling with Red color."
pub struct Red;

impl Color for Red {
    fn fill(&self) {
        println!("Filling with Red color.");
    }
}

// TODO: Define a struct Blue that implements Color
// - Implement the fill() method to print "Filling with Blue color."
pub struct Blue;

impl Color for Blue {
    fn fill(&self) {
        println!("Filling with Blue color.");
    }
}
