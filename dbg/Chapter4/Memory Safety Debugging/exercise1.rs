struct Data<'a> {  
    value: &'a str,  
}  


fn create_data() -> Data<'static> {  
    let text = String::from("Hello, world!");  
    // Here, we're trying to store a reference to a temporary value in Data.  
    Data { value: &text }  
}  


fn main() {  
    let data = create_data();  
    println!("{}", data.value);  
}
