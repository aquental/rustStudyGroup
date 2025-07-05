pub trait ProductComponent {
    fn show_details(&self);
}

// TODO: Define the Product struct
// - Should have private fields 'name' of type String and 'price' of type f64
// TODO: Implement the Product constructor
// TODO: Implement the ProductComponent trait for Product
//     Implement the show_details method in Product
//     Method should print "Product: {name}, Price: ${price}"

// TODO: Define the ProductBundle struct
// - Should have a private field 'products' which is a Vec<Box<dyn ProductComponent>>
// TODO: Implement the ProductBundle constructor
// TODO: Implement the add method in ProductBundle
// TODO: Implement the remove method in ProductBundle
// TODO: Implement the ProductComponent trait for ProductBundle
//     Implement the show_details method in ProductBundle
//     Method should print "Product Bundle:" followed by details of each product
