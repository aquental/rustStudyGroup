/**
Improve the readability of the function's logic.
The current code evaluates whether a given year is a leap year, but it's a bit more complex than it needs to be.
Refactor the code by simplifying the logic while maintaining its functionality.
Apply the KISS principle to enhance the simplicity and clarity of the function is_leap_year.
Simplify this logic to make it easy to read and understand.
**/
// Simplified leap year calculation following KISS principle
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    let year = 2024;
    let leap = is_leap_year(year);
    println!("{} is a leap year: {}", year, leap);
}
