struct Data {  
    value: String,  
}  


fn create_data() -> Data {  
    let text = String::from("Hello, world!");  
    Data { value: text }  
}  


fn main() {  
    let data = create_data();  
    println!("{}", data.value);  
}

