pub trait BrightnessStrategy {
    fn adjust_brightness(&self);
}

// TODO: Implement BrightStrategy struct that implements BrightnessStrategy
// - Implement adjust_brightness method to print "Adjusting brightness to bright."
pub struct BrightStrategy;
impl BrightnessStrategy for BrightStrategy {
    fn adjust_brightness(&self) {
        println!("Adjusting brightness to bright.");
    }
}

// TODO: Implement DimStrategy struct that implements BrightnessStrategy
// - Implement adjust_brightness method to print "Adjusting brightness to dim."
pub struct DimStrategy;
impl BrightnessStrategy for DimStrategy {
    fn adjust_brightness(&self) {
        println!("Adjusting brightness to dim.");
    }
}
