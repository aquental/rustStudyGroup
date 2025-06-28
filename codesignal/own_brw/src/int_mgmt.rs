fn transform_numbers(numbers: &mut Vec<i32>) {
    for num in numbers.iter_mut() {
        *num += 10;
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    transform_numbers(&mut numbers);
    for num in numbers.iter() {
        println!("{}", num);
    }
}
