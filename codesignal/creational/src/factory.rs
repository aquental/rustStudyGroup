mod document;
mod document_creator;

use document_creator::{ExcelDocumentCreator, WordDocumentCreator, PdfDocumentCreator};
// TODO: Import PdfDocumentCreator

fn main() {
    let creator: Box<dyn document_creator::DocumentCreator> = Box::new(WordDocumentCreator);
    let doc = creator.create_document();
    doc.open();

    let creator: Box<dyn document_creator::DocumentCreator> = Box::new(ExcelDocumentCreator);
    let doc = creator.create_document();
    doc.open();

    // TODO: Instantiate a PdfDocumentCreator
    let creator: Box<dyn document_creator::DocumentCreator> = Box::new(PdfDocumentCreator);
    // TODO: Create a PdfDocument using PdfDocumentCreator
    let doc = creator.create_document();
    // TODO: Call the open method on the created PdfDocument
    doc.open();
}
