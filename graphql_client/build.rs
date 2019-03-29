extern crate skeptic;

fn main() {
    // Generates doc tests for the readme.
    skeptic::generate_doc_tests(&["../README.md"]);
}
