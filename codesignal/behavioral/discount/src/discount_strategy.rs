pub trait DiscountStrategy {
    fn apply_discount(&self, price: f64) -> f64;
}

// TODO: Create the following three concrete strategy structs:
// - PercentageDiscount:
//   - Constructor parameter for percentage
//   - Implement apply_discount(price: f64) -> f64
//     - Applies percentage discount to given price
pub struct PercentageDiscount {
    percentage: f64,
}
impl PercentageDiscount {
    pub fn new(percentage: f64) -> PercentageDiscount {
        PercentageDiscount { percentage: percentage / 100.0 }
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        price * (1.0 - self.percentage)
    }
}

// - FixedAmountDiscount:
//   - Constructor parameter for amount
//   - Implement apply_discount(price: f64) -> f64
//     - Subtracts fixed amount from given price
pub struct FixedAmountDiscount {
    amount: f64,
}
impl FixedAmountDiscount {
    pub fn new(amount: f64) -> FixedAmountDiscount {
        FixedAmountDiscount { amount }
    }
}

impl DiscountStrategy for FixedAmountDiscount {
    fn apply_discount(&self, price: f64) -> f64 {
        price - self.amount
    }
}

// - BuyOneGetOneFree:
//   - Implement apply_discount(price: f64) -> f64
//     - Halves the given price
pub struct BuyOneGetOneFree;

impl BuyOneGetOneFree {
    pub fn new() -> BuyOneGetOneFree {
        BuyOneGetOneFree
    }
}

impl DiscountStrategy for BuyOneGetOneFree {
    fn apply_discount(&self, price: f64) -> f64 {
        price / 2.0
    }
}
