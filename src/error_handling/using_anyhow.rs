use super::common::Student;
use anyhow::Context;

pub fn read_student_from_file(path: &str) -> anyhow::Result<Student> {
    let file = std::fs::File::open(&path)?;
    let text = std::io::read_to_string(&file)?;
    let student = serde_json::from_str(&text)?;

    Ok(student)
}

/// using anyhow with context information
/// with_context vs context
/// https://stackoverflow.com/questions/65459952/what-is-the-difference-between-context-and-with-context-in-anyhow
fn read_student_from_file_with_context(path: &str) -> anyhow::Result<Student> {
    let file = std::fs::File::open(&path)
        .with_context(|| format!("failed to read file from ({path})"))?;

    let text = std::io::read_to_string(&file)?;

    let student = serde_json::from_str(&text)
        .with_context(|| format!("failed to deserialize from ({text})"))?;

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

    #[test]
    fn it_will_error_read_student_from_file_with_context() {
        let err = read_student_from_file_with_context("some.json")
            .unwrap_err();

        assert_eq!(format!("{}", err), "failed to read file from (some.json)");

        let err = err.downcast_ref::<std::io::Error>();

        assert!(err.is_some());
        assert_eq!(err.unwrap().kind(), std::io::ErrorKind::NotFound);
    }
}
