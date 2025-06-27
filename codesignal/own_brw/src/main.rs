fn change(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hi");
    change(&mut s);
    println!("After mutation: {}", s);
}
