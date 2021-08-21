#![cfg(test)]

fn check_program(path: &str) {
    let source = std::fs::read_to_string(path)
        .expect(&format!("failed to read {:?}", path));
    match interpreter::parse(&source) {
        Ok(_ast) => {}
        Err(_e) => panic!("file {:?} contains parse errors", path),
    }
}

#[test]
fn doc_gen() {
    check_program("../programs/doc-gen/main.tnt");
}

#[test]
fn doc_test() {
    check_program("../programs/doc-test/main.tnt");
}

#[test]
fn literate() {
    check_program("../programs/literate/main.tnt");
}
