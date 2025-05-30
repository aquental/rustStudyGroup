fn main() {  
    let mut data = vec![1, 2, 3];  
    let ref1 = &data[0];  
    println!("Before clear, ref1 points to: {}", ref1);  
    data.clear();  
    //println!("After clear, ref1 points to: {}", ref1); // AI warns: use-after-free  
}
