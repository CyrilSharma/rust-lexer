use std::process::Command;

#[test]
fn test_parse() {
        Command::new("target/debug/rflex")
            .arg("tests/data/integration/input")
            .arg("tests/tester/src/tokenizer.rs")
            .output()
            .expect("Failed to execute command");
        println!("Hello, world!");

        let output = Command::new("cargo")
            .arg("test")
            .current_dir("tests/tester")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Tester failed");
}
