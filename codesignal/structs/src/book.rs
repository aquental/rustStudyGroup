/*
Define a struct named Book with string fields title and author, and an integer (i32) field pages.
Implement associated functions and methods, including a constructor new and a reading_time method.
*/
// Define a struct named 'Book' with fields 'title', 'author' (both String), and 'pages' (i32)
struct Book {
    title: String,
    author: String,
    pages: i32,
}

// Implement methods for 'Book'
impl Book {
    // Associated function 'new' that takes title, author, and pages parameters and returns an instance of 'Book'
    fn new(title: String, author: String, pages: i32) -> Self {
        Self {
            title,
            author,
            pages,
        }
    }

    // 'reading_time' method that calculates reading time using the formula: pages / 50.0
    fn reading_time(&self) -> f32 {
        self.pages as f32 / 50.0
    }
}

fn main() {
    // Creating an instance of 'Book' using 'new' associated function
    let book = Book::new(
        String::from("Rust Programming"),
        String::from("John Doe"),
        200,
    );
    // Printing the estimated reading time
    println!("Estimated reading time: {} hours", book.reading_time());
}
