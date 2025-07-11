use std::path::Path;

// Define the strategy trait for parsing logic
trait FileParser {
    fn parse(&self, file_name: &str);
}

// Concrete strategy for CSV parsing
struct CsvParser;
impl FileParser for CsvParser {
    fn parse(&self, file_name: &str) {
        println!("Parsing CSV file: {}", file_name);
    }
}

// Concrete strategy for XML parsing
struct XmlParser;
impl FileParser for XmlParser {
    fn parse(&self, file_name: &str) {
        println!("Parsing XML file: {}", file_name);
    }
}

// Concrete strategy for JSON parsing
struct JsonParser;
impl FileParser for JsonParser {
    fn parse(&self, file_name: &str) {
        println!("Parsing JSON file: {}", file_name);
    }
}

// Context that holds the strategy and delegates parsing
struct FileProcessor {
    parser: Box<dyn FileParser>,
}

impl FileProcessor {
    // Create a new FileProcessor with the appropriate parser based on file extension
    fn new(file_name: &str) -> Option<Self> {
        let extension = Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        let parser: Box<dyn FileParser> = match extension {
            Some(ext) => match ext.as_str() {
                "csv" => Box::new(CsvParser),
                "xml" => Box::new(XmlParser),
                "json" => Box::new(JsonParser),
                _ => return None, // Unsupported file type
            },
            None => return None, // No extension
        };

        Some(FileProcessor { parser })
    }

    // Execute the parsing logic
    fn process(&self, file_name: &str) {
        self.parser.parse(file_name);
    }
}

fn main() {
    let files = vec!["file.csv", "file.xml", "file.json", "file.txt"];

    for file_name in files {
        match FileProcessor::new(file_name) {
            Some(processor) => {
                processor.process(file_name);
            }
            None => println!("Unsupported file type: {}", file_name),
        }
    }
}
