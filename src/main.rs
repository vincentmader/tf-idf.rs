use clap::Parser;
use sorted_tf_idf_list::cli_parser::CliParser;
use sorted_tf_idf_list::document::Document;
use sorted_tf_idf_list::tf_idf;
use std::collections::HashMap;
use std::time::Instant;

fn get_tfidf_scores() {
    // Get CLI arguments.
    let args = CliParser::parse();
    let paths_in = &args.paths_in();
    let path_out = &args.path_out();

    // Get list of documents.
    let documents: Vec<Document> = paths_in.iter().map(|p| Document::from(p)).collect();

    // Get list of terms.
    let mut terms = vec![];
    for document in documents.iter() {
        // terms = [terms, document.terms().to_owned()].concat();
        for term in document.terms().iter() {
            terms.push(term);
        }
    }

    // Get HashMap of term-frequencies (for each term).
    let mut term_frequencies = HashMap::new();
    for term in terms.iter() {
        let mut foo = HashMap::new();
        for document in documents.iter() {
            let term_frequency = tf_idf::term_frequency(term, document);
            foo.insert(document.path(), term_frequency);
        }
        term_frequencies.insert(term, foo);
    }

    // Get HashMap of inverse-document-frequencies (for each term, for each document).
    let mut inverse_document_frequencies = HashMap::new();
    for term in terms.iter() {
        let inverse_document_frequency = tf_idf::inverse_document_frequency(term, &documents);
        inverse_document_frequencies.insert(term, inverse_document_frequency);
    }

    // Get HashMap of term-frequencies-inverse-document-frequencies
    // (for each term, for each document).
    let mut tf_idfs = HashMap::new();
    for term in terms.iter() {
        let inverse_document_frequency = inverse_document_frequencies.get(term).unwrap();
        let mut foo = HashMap::new();
        for document in documents.iter() {
            let term_frequency = term_frequencies
                .get(term)
                .unwrap()
                .get(document.path())
                .unwrap();
            let tf_idf = term_frequency * inverse_document_frequency;
            foo.insert(document.path(), tf_idf);
        }
        tf_idfs.insert(term.to_string(), foo);
    }

    // Save results to file.
    std::fs::write(path_out, serde_json::to_string_pretty(&tf_idfs).unwrap()).unwrap();
}

fn main() {
    let start = Instant::now();
    get_tfidf_scores();
    let duration = start.elapsed();
    println!("{:?}", duration);
}
