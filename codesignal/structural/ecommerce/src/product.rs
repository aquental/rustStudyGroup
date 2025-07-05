pub trait ProductComponent {
    fn show_details(&self);
}

// TODO: Define the Product struct
// - Should have private fields 'name' of type String and 'price' of type f64
// TODO: Implement the Product constructor
// TODO: Implement the ProductComponent trait for Product
//     Implement the show_details method in Product
//     Method should print "Product: {name}, Price: ${price}"
pub struct Product {
    name: String,
    price: f64,
}
impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product { name, price }
    }
}
impl ProductComponent for Product {
    fn show_details(&self) {
        println!("Product: {}, Price: ${}", self.name, self.price);
    }
}

// TODO: Define the ProductBundle struct
// - Should have a private field 'products' which is a Vec<Box<dyn ProductComponent>>
// TODO: Implement the ProductBundle constructor
// TODO: Implement the add method in ProductBundle
// TODO: Implement the remove method in ProductBundle
// TODO: Implement the ProductComponent trait for ProductBundle
//     Implement the show_details method in ProductBundle
//     Method should print "Product Bundle:" followed by details of each product
pub struct ProductBundle {
    products: Vec<Box<dyn ProductComponent>>,
}
impl ProductBundle {
    pub fn new() -> ProductBundle {
        ProductBundle { products: vec![] }
    }
    pub fn add(&mut self, product: Box<dyn ProductComponent>) {
        self.products.push(product);
    }
    pub fn remove(&mut self, index: usize) {
        self.products.remove(index);
    }
}
impl ProductComponent for ProductBundle {
    fn show_details(&self) {
        println!("Product Bundle:");
        for product in &self.products {
            product.show_details();
        }
    }
}
