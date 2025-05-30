fn process(data: Vec<String>) {  
    let _ = data.iter().map(|x| x + 1); // AI flags this mistake  
}  

fn main() {  
    let words = vec![String::from("hello"), String::from("world")];  
    process(words);  
}
