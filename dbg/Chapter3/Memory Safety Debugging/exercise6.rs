struct DataHolder<'a> {
    values: Vec<i32>,
    reference: &'a i32,
}
impl<'a> DataHolder<'a> {
    fn new(reference: &'a i32) -> Self {
        Self { values: Vec::new(), reference }
    }
    fn add_and_reference(&mut self, value: i32) -> &i32 {
        self.values.push(value);
        self.values.last().unwrap()
    }
}
fn main() {
    let value = 10;
    let mut holder = DataHolder::new(&value);
    let reference = holder.add_and_reference(42);
    // // This push might cause a reallocation, invalidating `reference`
    holder.values.push(100);


    println!("Referenced value: {}", reference);
}
