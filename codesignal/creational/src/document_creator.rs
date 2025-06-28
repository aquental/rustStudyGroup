use crate::document::{Document, ExcelDocument, WordDocument, PdfDocument};
// TODO: Import PdfDocument

pub trait DocumentCreator {
    fn create_document(&self) -> Box<dyn Document>;
}

pub struct WordDocumentCreator;

impl DocumentCreator for WordDocumentCreator {
    fn create_document(&self) -> Box<dyn Document> {
        Box::new(WordDocument)
    }
}

pub struct ExcelDocumentCreator;

impl DocumentCreator for ExcelDocumentCreator {
    fn create_document(&self) -> Box<dyn Document> {
        Box::new(ExcelDocument)
    }
}

// TODO: Add the new PdfDocumentCreator struct here with create_document method, that returns a new PdfDocument.
pub struct PdfDocumentCreator;

impl DocumentCreator for PdfDocumentCreator {
    fn create_document(&self) -> Box<dyn Document> {
        Box::new(PdfDocument)
    }
}
