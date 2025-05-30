fn process_payment(amount: f64, currency: &str) -> Result<(), String> {  
    if amount < 0.0 {  
        return Err("Negative amount not allowed".to_string());  
    }  
    println!("Processing ${} in {}", amount, currency);  
    if currency != "USD" {  
        return Err("Only USD is supported".to_string());  
    }  
    Ok(())  
}  

fn main() {  
    match process_payment(-10.0, "USD") {  
        Ok(()) => println!("Payment processed"),  
        Err(err) => println!("Error: {}", err),  
    }  
    match process_payment(10.0, "USD") {  
        Ok(()) => println!("Payment processed"),  
        Err(err) => println!("Error: {}", err),  
    }  
}
