#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    fn new(name: &str, price: f64, stock: u32) -> Product {
        Product {
            name: name.to_string(),
            price,
            stock,
        }
    }

    fn format_price(&self) -> String {
        format!("Price: {:.2}", self.price)
    }

    fn find_expensive(products: Vec<Product>) -> Option<Product> {
        products.into_iter().max_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal))
    }

    fn get_product_names(products: &[Product]) -> Vec<String> {
        products.iter().map(|p| p.name.clone()).collect()
    }
}

fn main() {
    let p1 = Product::new("Laptop", 999.99, 10);
    let p2 = Product::new("Mouse", 29.99, 50);
    let products = vec![p1, p2];
    
    println!("Expensive: {:?}", Product::find_expensive(products.clone()));
    println!("Formatted price of first product: {}", products[0].format_price());
    println!("Names: {:?}", Product::get_product_names(&products));
}
