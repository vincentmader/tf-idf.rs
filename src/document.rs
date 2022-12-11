use crate::file_io;
use std::path::PathBuf;

pub struct Document {
    path: PathBuf,
    terms: Vec<String>,
}

impl Document {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn terms(&self) -> &Vec<String> {
        &self.terms
    }
}

impl From<&PathBuf> for Document {
    fn from(path: &PathBuf) -> Self {
        let content = file_io::read_file_to_string(path);

        let mut terms = vec![];
        for line in content.lines() {
            for term in line.split(" ") {
                if term.len() == 0 {
                    continue;
                }
                let term = remove_punctuation(term);
                terms.push(term);
            }
        }

        Document {
            path: path.to_owned(),
            terms: terms,
        }
    }
}

fn remove_punctuation(term: &str) -> String {
    term.replace(
        &['(', ')', ',', '\"', '.', ';', '“', '”', ':', '\'', '?', '!'][..],
        "",
    )
}
