#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    score: u8
}

impl Student {
    pub fn new(name: impl Into<String>, score: u8) -> Self {
        Self {
            name: name.into(),
            score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Student;

    #[test]
    fn it_same_struct() {
        let s_1 = Student {
            name: String::from("abc"),
            score: 20
        };

        let s_2 = Student::new("abc", 20);

        assert_eq!(s_1, s_2);
    }
}
