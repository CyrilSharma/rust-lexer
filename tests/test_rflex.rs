use std::{process::Command, path::Path};

#[test]
fn test_parse() {
    let mut i = 0;
    while Path::new(&format!("tests/tester/data/input/{i}.tk")).exists() {
        Command::new("target/debug/rflex")
            .arg("tests/data/integration/input")
            .arg("tests/tester/src/tokenizer.rs")
            .output()
            .expect("Failed to execute command");

        let output = Command::new("cargo")
            .arg("run")
            .arg(format!("tests/tester/data/input/{i}.tk"))
            .arg(format!("tests/tester/data/output/{i}.tk"))
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Tester failed");
        i += 1;
    }
}
