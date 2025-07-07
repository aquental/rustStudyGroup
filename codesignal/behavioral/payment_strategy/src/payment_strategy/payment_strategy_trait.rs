pub trait PaymentStrategy {
    fn pay(&self, amount: u32);
}
