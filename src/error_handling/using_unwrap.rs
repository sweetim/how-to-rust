use super::common::Student;

pub fn read_student_from_file(path: &str) -> Student {
    let file = std::fs::File::open(path).unwrap();
    let text = std::io::read_to_string(&file).unwrap();

    serde_json::from_str(&text).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_will_panic_read_student_from_file() {
        read_student_from_file("some.json");
    }
}
