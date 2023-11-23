use std::fmt::{Display, Formatter};

use super::common::Student;

#[derive(Debug)]
enum ReadStudentFromFileError {
    FileError(std::io::Error),
    DeserializationError(serde_json::Error),
}

impl Display for ReadStudentFromFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "read student error ({:?})", self)
    }
}

impl std::error::Error for ReadStudentFromFileError {
}

fn read_student_from_file(path: &str) -> Result<Student, ReadStudentFromFileError> {
    let file = std::fs::File::open(&path)
        .map_err(ReadStudentFromFileError::FileError)?;

    let text = std::io::read_to_string(&file)
        .map_err(ReadStudentFromFileError::FileError)?;

    let student = serde_json::from_str(&text)
        .map_err(ReadStudentFromFileError::DeserializationError)?;

    Ok(student)
}

fn read_student_from_file_with_dynamic_trait(path: &str) -> Result<Student, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(&path)
        .map_err(ReadStudentFromFileError::FileError)?;

    let text = std::io::read_to_string(&file)
        .map_err(ReadStudentFromFileError::FileError)?;

    let student = serde_json::from_str(&text)
        .map_err(ReadStudentFromFileError::DeserializationError)?;

    Ok(student)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_error_read_student_from_file() {
        let err = read_student_from_file("some.json")
            .unwrap_err();

        assert_eq!(format!("{}", err), "read student error (FileError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))");
    }

    #[test]
    fn it_will_error_read_student_from_file_with_dynamic_trait() {
        let err = read_student_from_file_with_dynamic_trait("some.json")
            .unwrap_err();

        assert_eq!(format!("{}", err), "read student error (FileError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))");

        let err = err.downcast_ref::<ReadStudentFromFileError>();
        assert!(err.is_some());
    }
}
