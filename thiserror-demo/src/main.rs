use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error occurred")]
    IoError(#[from] std::io::Error),
}

fn read_file(path: &str) -> Result<String, MyError> {
    if path.is_empty() {
        return Err(MyError::InvalidInput("empty path".to_string()));
    }
    std::fs::read_to_string(path).map_err(MyError::IoError)
}

fn main() {
    let path = "./README.mdx";
    let result = read_file(path);
    println!("{:?}", result);
}
