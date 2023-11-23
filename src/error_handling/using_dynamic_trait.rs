use super::common::Student;

pub fn read_student_from_file(path: &str) -> Result<Student, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(&path)?;
    let text = std::io::read_to_string(&file)?;
    let student = serde_json::from_str(&text)?;

    Ok(student)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_error_read_student_from_file() {
        let err = read_student_from_file("some.json")
            .unwrap_err();

        assert_eq!(format!("{}", err), "No such file or directory (os error 2)");

        let err = err.downcast_ref::<std::io::Error>();

        assert!(err.is_some());
        assert_eq!(err.unwrap().kind(), std::io::ErrorKind::NotFound);
    }
}
