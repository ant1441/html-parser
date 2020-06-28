use std::rc::Rc;
use std::io;

use html_parser::dom::{Comment, Document, DocumentType, Element};

extern crate pretty_env_logger;

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let mut document = Document::new();

    let node = Comment::new("some comment".to_string());
    document.push_comment(node);

    let document_type = DocumentType::new("html".to_string(), String::new(), String::new());
    document.add_document_type(document_type);

    let node = Element::new_html("tag1".parse().unwrap());
    document.push_element(Rc::clone(&node));

    let node = Element::new_html("tag2".parse().unwrap());
    document.push_element(Rc::clone(&node));

    println!("Document: {:#?}", document);

    Ok(())
}
