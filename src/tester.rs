use std::fs;

struct tester {}
impl tester {
    fn test(path: String, items: &[(fn(String, String) -> bool, String)]) -> Vec<bool> {
        let mut res: Vec<bool> = Vec::new();
        let entries = fs::read_dir(format!("{path}/input")).expect("Invalid Directory");
        for entry in entries {
            if entry.is_err() { panic!("Invalid Directory"); }
            let os_str = entry.unwrap().file_name();
            let file_name = os_str.to_str().unwrap();
            let ident = file_name
                .trim()
                .replace('\n', "")
                .split("-")
                .map(|s| s.to_string())
                .nth(0)
                .expect("Filename should have non-zero length");
            for (func, label) in items {
                if *label == ident {
                    res.push(func(
                        format!("{path}/input/{file_name}"),
                        format!("{path}/output/{file_name}"),
                    ));
                }
            }
            panic!("One of your labels is wrong!");
        }
        return res;
    }
}