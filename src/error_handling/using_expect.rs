use super::common::Student;

pub fn read_student_from_file(path: &str) -> Student {
    let file = std::fs::File::open(&path).expect(format!("file not found {path}").as_str());

    let text =
        std::io::read_to_string(&file).expect(format!("fail to read from ({path})").as_str());

    serde_json::from_str(&text)
        .expect(format!("failed to deserialize into json from ({path})").as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "file not found some.json")]
    fn it_will_panic_read_student_from_file() {
        read_student_from_file("some.json");
    }
}
