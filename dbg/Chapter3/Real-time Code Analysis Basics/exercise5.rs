fn main() { 
    let data: Vec<i32> = vec![1, 2, 3, 4, 5]; // Create a vector of integers  
    let filtered: Vec<i32> = data // Declare a new vector to store filtered results  
    .iter() // Borrow an iterator over the vector's elements  
    .filter(|&&x| x > 3) // Keep only numbers greater than 3  
    .map(|&x| x * 2) // Double each remaining number  
    .collect(); // Collect the results into a new Vec<i32>  
    println!("{:?}", filtered); // Print the filtered vector  
    let optimized: Vec<i32> = vec![1, 2, 3, 4, 5] // Declare an optimized vector  
    .into_iter() // Take ownership of the vector's elements with an iterator  
    .filter(|x| *x > 3) // Keep only numbers greater than 3  
    .map(|x| x * 2) // Double each remaining number  
    .collect(); // Collect the results into a new Vec<i32>  
    println!("{:?}", optimized); // Print the optimized vector
}
