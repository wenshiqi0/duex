mod context;
mod isolate;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use crate::isolate::Isolate;

    #[test]
    fn test_run_code() {
        let mut file = File::open("../example/index.ts").unwrap();
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_size) => {
                // js vm
                let isolate = Isolate::new();
            }
            Err(_) => panic!("read file failed!"),
        }
    }
}
