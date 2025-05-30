fn check_number(num: i32) -> Result<(), &'static str> { // Define a function checking a number, returning a Result  
    if num >= 0 { // Check if the number is non-negative  
        Ok(()) // Return Ok if the condition is true  
    } else { // Handle the alternative case  
        Err("Number is negative!") // Return an error message if negative  
    } // Close the else block  
} // Close the function  
fn main() {  
    let result: Result<(), &str> = check_number(5); // Corrected call  
    match result {  
        Ok(_) => println!("All good!"),  
        Err(msg) => println!("Error: {}", msg),  
    }  
}

