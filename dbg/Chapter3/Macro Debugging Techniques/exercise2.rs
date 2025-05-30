macro_rules! build_struct {  
    ($name:ident, $($field:ident: $type:ty),*) => { // Define the macro with a flexible field list  
        struct $name { // Create a struct with the given name  
            $($field: $type),* // Expand multiple fields with their types  
        }  
    };  
}  

fn main() {  
    build_struct!(User, name: String, age: u32, active: bool); // Build a User struct with 3 fields  
    let user = User { // Initialize a User instance  
        name: String::from("Alice"), // Set name as a String  
        age: 30, // Set age as a u32  
        active: true // Set active as a bool  
    };  
    println!("User: {}, Age: {}, Active: {}", user.name, user.age, user.active); // Print the values  
}
