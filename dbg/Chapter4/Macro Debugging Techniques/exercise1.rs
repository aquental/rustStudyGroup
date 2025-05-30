macro_rules! build_struct {  
    ($name:ident, $field:ident) => {  
        struct $name {  
            $field: String,  
        }  
    };  
}  

fn main() {  
    build_struct!(User, name); // Creates: struct User { name: String }  
}
