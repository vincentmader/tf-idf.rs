use crate::document::Document;

pub fn term_frequency(term: &String, document: &Document) -> f64 {
    let mut term_frequency = 0.;
    let document_terms = &document.terms();
    for document_term in document_terms.iter() {
        if term == document_term {
            term_frequency += 1.;
        }
    }
    let term_frequency = term_frequency / document_terms.len() as f64;
    term_frequency
}

pub fn inverse_document_frequency(term: &String, documents: &Vec<Document>) -> f64 {
    let mut documents_containing_term = 0.;
    for document in documents.iter() {
        let document_terms = &document.terms();
        if document_terms.contains(term) {
            documents_containing_term += 1.;
        }
    }
    let inverse_document_frequency = documents.len() as f64 / documents_containing_term;
    let inverse_document_frequency = inverse_document_frequency.log(10.);
    inverse_document_frequency
}
