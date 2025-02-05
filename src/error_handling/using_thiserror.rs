use super::common::Student;

#[derive(Debug, thiserror::Error)]
pub enum ReadStudentFromFileThisError {
    #[error("failed to read file from ({path}) {source}")]
    FileError {
        path: String,
        source: std::io::Error,
    },
    #[error("failed to deserialize content from ({0})")]
    DeserializationError(serde_json::Error),
}

pub fn read_student_from_file(path: &str) -> Result<Student, ReadStudentFromFileThisError> {
    let file =
        std::fs::File::open(path).map_err(|err| ReadStudentFromFileThisError::FileError {
            path: String::from(path),
            source: err,
        })?;

    let text =
        std::io::read_to_string(&file).map_err(|err| ReadStudentFromFileThisError::FileError {
            path: String::from(path),
            source: err,
        })?;

    let student =
        serde_json::from_str(&text).map_err(ReadStudentFromFileThisError::DeserializationError)?;

    Ok(student)
}

#[derive(Debug, thiserror::Error)]
pub enum ReadStudentFromFileThisErrorUsingFrom {
    #[error("failed to read file {source}")]
    FileError {
        #[from]
        source: std::io::Error,
    },
    #[error("failed to deserialize content from ({0})")]
    DeserializationError(#[from] serde_json::Error),
}
pub fn read_student_from_file_using_from(
    path: &str,
) -> Result<Student, ReadStudentFromFileThisErrorUsingFrom> {
    let file = std::fs::File::open(path)?;
    let text = std::io::read_to_string(&file)?;
    let student = serde_json::from_str(&text)?;

    Ok(student)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_will_error_read_student_from_file() {
        let err = read_student_from_file("some.json").unwrap_err();

        assert_eq!(
            format!("{}", err),
            "failed to read file from (some.json) No such file or directory (os error 2)"
        );
    }

    #[test]
    fn it_will_error_read_student_from_file_using_from() {
        let err = read_student_from_file_using_from("some.json").unwrap_err();

        assert_eq!(
            format!("{}", err),
            "failed to read file No such file or directory (os error 2)"
        );
    }
}
