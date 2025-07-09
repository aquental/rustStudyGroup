// TODO: Define the ClimateStrategy trait
// - Define the adjust method
pub trait ClimateStrategy {
    fn adjust(&self);
}

// TODO: Implement the HumidityStrategy struct
// - Implement ClimateStrategy for HumidityStrategy
//   - adjust method prints "Adjusting humidity levels."
pub struct HumidityStrategy;

impl ClimateStrategy for HumidityStrategy {
    fn adjust(&self) {
        println!("Adjusting humidity levels.");
    }
}

// TODO: Implement the AirQualityStrategy struct
// - Implement ClimateStrategy for AirQualityStrategy
//   - adjust method prints "Purifying air quality."
pub struct AirQualityStrategy;

impl ClimateStrategy for AirQualityStrategy {
    fn adjust(&self) {
        println!("Purifying air quality.");
    }
}

// TODO: Define the ClimateControl struct
// - strategy: Box<dyn ClimateStrategy>
pub struct ClimateControl {
    strategy: Box<dyn ClimateStrategy>,
}
// TODO: Implement methods for ClimateControl
// - new(strategy: Box<dyn ClimateStrategy>) -> Self
// - set_strategy(&mut self, strategy: Box<dyn ClimateStrategy>)
// - execute(&self)
impl ClimateControl {
    pub fn new(strategy: Box<dyn ClimateStrategy>) -> Self {
        Self { strategy }
    }
    pub fn set_strategy(&mut self, strategy: Box<dyn ClimateStrategy>) {
        self.strategy = strategy;
    }
    pub fn execute(&self) {
        self.strategy.adjust();
    }
}
