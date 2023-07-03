use std::{process::{Command, Stdio}, path::Path};

#[test]
fn test_parse() {
    let mut i = 0;
    while Path::new(&format!("tests/tester/data/gen-{i}.tk")).exists() {
        let tokpath = "tests/tester/src/tokenizer.rs";
        let genpath = &format!("tests/tester/data/gen-{i}.tk");
        let out1 = Command::new("target/debug/rflex")
            .arg(genpath)
            .arg(tokpath)
            .output()
            .expect("Failed to execute command");
        assert!(out1.status.success(), "Generator Failed");
        
        let inpath = &format!("data/in-{i}.txt");
        let outpath = &format!("data/out-{i}.txt");
        let out2 = Command::new("cargo")
            .arg("run")
            .arg(inpath)
            .arg(outpath)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .current_dir("tests/tester")
            .output()
            .expect("Failed to execute command");        
        if out2.status.success() {
            println!("{}", String::from_utf8_lossy(&out2.stdout));
            assert!(0 == out2.status.code().unwrap());
        } else {
            println!("{}", String::from_utf8_lossy(&out2.stdout));
            assert!(false, "Parsing Failed");
        }
        i += 1;
    }
}
