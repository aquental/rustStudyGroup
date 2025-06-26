struct Rectangle<T> {
    width: T,
    height: T,
}

// Implementing methods for numeric types that support multiplication
impl<T> Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let int_rect = Rectangle::new(5, 10);
    let float_rect = Rectangle::new(5.5, 10.2);

    println!("Integer Rectangle area: {}", int_rect.area());
    println!("Float Rectangle area: {}", float_rect.area());
}
