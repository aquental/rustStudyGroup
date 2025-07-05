use crate::product::ProductComponent;

// TODO: Define the DiscountFeature struct
// - Should have a private field 'product' of type Box<dyn ProductComponent>
// - Should have a private field 'discount' of type f64
// TODO: Implement the DiscountFeature constructor
// TODO: Implement the ProductComponent trait for DiscountFeature
//     Implement the show_details method in DiscountFeature
//     Method should call self.product.show_details() and then print "Discount: ${discount}"

// TODO: Define the GiftWrapFeature struct
// - Should have a private field 'product' of type Box<dyn ProductComponent>
// TODO: Implement the GiftWrapFeature constructor
// TODO: Implement the ProductComponent trait for GiftWrapFeature
//     Implement the show_details method in GiftWrapFeature
//     Method should call self.product.show_details() and then print "Includes gift wrapping."
