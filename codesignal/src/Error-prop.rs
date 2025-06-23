/**
Transition from using panic! to using Rust's Result type and the '?' operator for error propagation. The current codebase uses unchecked errors with panic!, leading to less robust and less readable code. Refactor the code by using Result, define meaningful error types, and use the '?' operator to handle errors cleanly, making the code more idiomatic and maintainable.
The application simulates a library system that manages book borrowing operations. The current implementation handles errors by panicking, but our goal is to refactor it to use the Result construct, custom error types, and proper error propagation to improve code quality and readability.
**/
use std::fmt;

// Defining custom error types with contextual information
#[derive(Debug)]
enum RepositoryError {
    BookNotFound { book_id: String },
    InvalidUser { user_id: String },
    UpdateFailed { book_id: String },
}

#[derive(Debug)]
enum ServiceError {
    BookUnavailable { book_id: String },
    RepositoryError(RepositoryError),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::BookNotFound { book_id } => {
                write!(f, "Book not found: {}", book_id)
            }
            RepositoryError::InvalidUser { user_id } => {
                write!(f, "Invalid user: {}", user_id)
            }
            RepositoryError::UpdateFailed { book_id } => {
                write!(f, "Failed to update book status: {}", book_id)
            }
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::BookUnavailable { book_id } => {
                write!(f, "Book is not available: {}", book_id)
            }
            ServiceError::RepositoryError(err) => write!(f, "Repository error: {}", err),
        }
    }
}

// Implementing error conversion from RepositoryError to ServiceError
impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> ServiceError {
        ServiceError::RepositoryError(err)
    }
}

struct BookRepository;

impl BookRepository {
    fn is_book_available(&self, book_id: &str) -> Result<bool, RepositoryError> {
        if book_id == "00000" {
            return Err(RepositoryError::BookNotFound {
                book_id: book_id.to_string(),
            });
        }
        Ok(true)
    }

    fn is_valid_user(&self, user_id: &str) -> Result<bool, RepositoryError> {
        if user_id != "user1" {
            return Err(RepositoryError::InvalidUser {
                user_id: user_id.to_string(),
            });
        }
        Ok(true)
    }

    fn update_book_status(&self, book_id: &str, is_available: bool) -> Result<(), RepositoryError> {
        if book_id == "error" {
            return Err(RepositoryError::UpdateFailed {
                book_id: book_id.to_string(),
            });
        }
        println!(
            "Book status updated: {}",
            if is_available { "Available" } else { "Not available" }
        );
        Ok(())
    }
}

struct LibraryService {
    book_repository: BookRepository,
}

impl LibraryService {
    fn borrow_book(&self, book_id: &str, user_id: &str) -> Result<(), ServiceError> {
        if !self.book_repository.is_book_available(book_id)? {
            return Err(ServiceError::BookUnavailable {
                book_id: book_id.to_string(),
            });
        }
        self.book_repository.is_valid_user(user_id)?;
        self.book_repository.update_book_status(book_id, false)?;
        println!("Book borrowed successfully!");
        Ok(())
    }
}

fn main() {
    let book_repository = BookRepository;
    let library_service = LibraryService { book_repository };

    // Demonstrating error handling with Result
    match library_service.borrow_book("12345", "user1") {
        Ok(_) => println!("Transaction completed"),
        Err(e) => println!("Error: {}", e),
    }

    // Example with invalid book ID
    match library_service.borrow_book("00000", "user1") {
        Ok(_) => println!("Transaction completed"),
        Err(e) => println!("Error: {}", e),
    }

    // Example with invalid user
    match library_service.borrow_book("12345", "invalid_user") {
        Ok(_) => println!("Transaction completed"),
        Err(e) => println!("Error: {}", e),
    }
}
