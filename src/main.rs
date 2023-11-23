use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Student {
    is_graduated: bool,
}

fn read_student_from_file_1(path: &str) -> Result<Student, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(&path)?;
    let text = std::io::read_to_string(&file)?;
    let student = serde_json::from_str(&text)?;

    Ok(student)
}

/// using anyhow the simplest version
fn read_student_from_file_2(path: &str) -> anyhow::Result<Student> {
    let file = std::fs::File::open(&path)?;
    let text = std::io::read_to_string(&file)?;
    let student = serde_json::from_str(&text)?;

    Ok(student)
}

/// using anyhow with context information
/// with_context vs context
/// https://stackoverflow.com/questions/65459952/what-is-the-difference-between-context-and-with-context-in-anyhow
fn read_student_from_file_3(path: &str) -> anyhow::Result<Student> {
    let file = std::fs::File::open(&path)
        .with_context(|| format!("failed to read file from ({path})"))?;

    let text = std::io::read_to_string(&file)?;

    let student = serde_json::from_str(&text)
        .with_context(|| format!("failed to deserialize from ({text})"))?;

    Ok(student)
}

// fn read_student_from_file_4(path: &str) -> anyhow::Result<Student> {
//     let file = std::fs::File::open(&path)
//         .with_context(|| format!("failed to read file from ({path})"))?;
//
//     let text = std::io::read_to_string(&file)?;
//
//     let student = serde_json::from_str(&text)
//         .with_context(|| format!("failed to deserialize from ({text})"))?;
//
//     Ok(student)
// }


fn main() {
    println!("using box dyn Error");
    match read_student_from_file_1("some.json") {
        Ok(student) => println!("{:?}", student),
        Err(err) => println!("{:?}", err),
    };

    println!("using anyhow crate");
    match read_student_from_file_2("some.json") {
        Ok(student) => println!("{:?}", student),
        Err(err) => println!("{:?}", err),
    };

    println!("using anyhow crate with error context");
    match read_student_from_file_3("some.json") {
        Ok(student) => println!("{:?}", student),
        Err(err) => println!("{:?}", err),
    };

    println!("using anyhow crate with error context and retrieve error");
    match read_student_from_file_3("some.json") {
        Ok(student) => println!("{:?}", student),
        Err(err) => {
            if let Some(err) = err.downcast_ref::<std::io::Error>() {
                match err.kind() {
                    std::io::ErrorKind::NotFound => println!("not found"),
                    _ => println!("default")
                }
            }

            println!("{:?}", err);
        },
    };
}

#[cfg(test)]
mod tests {
    fn test_read_student_from_file() {

    }
}
