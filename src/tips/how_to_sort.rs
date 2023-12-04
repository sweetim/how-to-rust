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
    use super::*;

    fn get_sample() -> Vec<Student> {
        vec![
            Student::new("aa", 60),
            Student::new("ca", 50),
            Student::new("ab", 80),
            Student::new("ca", 10),
            Student::new("aa", 90),
            Student::new("ba", 20),
            Student::new("aa", 80),
            Student::new("ab", 90),
        ]
    }

    #[test]
    fn it_sort_1d() {
        let mut sample = get_sample();

        sample.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(sample, vec![
            Student::new("aa", 60),
            Student::new("aa", 90),
            Student::new("aa", 80),
            Student::new("ab", 80),
            Student::new("ab", 90),
            Student::new("ba", 20),
            Student::new("ca", 50),
            Student::new("ca", 10),
        ])
    }

    #[test]
    fn it_sort_2d_stable() {
        let mut sample = get_sample();

        sample.sort_by(|a, b| {
            match a.name.cmp(&b.name) {
                std::cmp::Ordering::Equal => a.score.cmp(&b.score),
                other => other,
            }
        });

        assert_eq!(sample, vec![
            Student::new("aa", 60),
            Student::new("aa", 80),
            Student::new("aa", 90),
            Student::new("ab", 80),
            Student::new("ab", 90),
            Student::new("ba", 20),
            Student::new("ca", 10),
            Student::new("ca", 50),
        ])
    }

    #[test]
    fn it_sort_2d_unstable() {
        let mut sample = get_sample();

        sample.sort_unstable_by(|a, b| {
            match a.name.cmp(&b.name) {
                std::cmp::Ordering::Equal => a.score.cmp(&b.score),
                other => other,
            }
        });

        assert_eq!(sample, vec![
            Student::new("aa", 60),
            Student::new("aa", 80),
            Student::new("aa", 90),
            Student::new("ab", 80),
            Student::new("ab", 90),
            Student::new("ba", 20),
            Student::new("ca", 10),
            Student::new("ca", 50),
        ])
    }
}
