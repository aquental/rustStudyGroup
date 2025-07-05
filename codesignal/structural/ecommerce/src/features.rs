use crate::product::ProductComponent;

// TODO: Define the DiscountFeature struct
// - Should have a private field 'product' of type Box<dyn ProductComponent>
// - Should have a private field 'discount' of type f64
// TODO: Implement the DiscountFeature constructor
// TODO: Implement the ProductComponent trait for DiscountFeature
//     Implement the show_details method in DiscountFeature
//     Method should call self.product.show_details() and then print "Discount: ${discount}"
pub struct DiscountFeature {
    product: Box<dyn ProductComponent>,
    discount: f64,
}
impl DiscountFeature {
    pub fn new(product: Box<dyn ProductComponent>, discount: f64) -> DiscountFeature {
        DiscountFeature { product, discount }
    }
}

impl ProductComponent for DiscountFeature {
    fn show_details(&self) {
        self.product.show_details();
        println!("Discount: ${}", self.discount);
    }
}

// TODO: Define the GiftWrapFeature struct
// - Should have a private field 'product' of type Box<dyn ProductComponent>
// TODO: Implement the GiftWrapFeature constructor
// TODO: Implement the ProductComponent trait for GiftWrapFeature
//     Implement the show_details method in GiftWrapFeature
//     Method should call self.product.show_details() and then print "Includes gift wrapping."
pub struct GiftWrapFeature {
    product: Box<dyn ProductComponent>,
}
impl GiftWrapFeature {
    pub fn new(product: Box<dyn ProductComponent>) -> GiftWrapFeature {
        GiftWrapFeature { product }
    }
}
impl ProductComponent for GiftWrapFeature {
    fn show_details(&self) {
        self.product.show_details();
        println!("Includes gift wrapping.");
    }
}
