fn check_number(num: i32) -> Result<(), &'static str> {  
    if num >= 0 {  
        Ok(())  
    } else {  
        Err("Number is negative!")  
    }  
}  

fn main() {  
    let result: Result<(), &str> = check_number(num: 5); // AI flags an issue here  
    match result {  
        Ok(_) => println!("All good!"),  
        Err(msg) => println!("Error: {}", msg),  
    }  
}
