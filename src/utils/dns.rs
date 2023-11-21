pub fn decode(name: &str) -> String {
    let mut labels: Vec<&str> = Vec::new();
    let mut idx = 0;
    loop {
        let len = name.as_bytes()[idx] as usize;
        if len == 0 {
            break;
        }
        labels.push(&name[idx + 1..idx + len + 1]);
        idx += len + 1;
    }

    labels.join(".")
}
