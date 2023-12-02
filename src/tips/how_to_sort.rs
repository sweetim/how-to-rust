#[derive(Debug, PartialEq)]
pub struct Student {
    name: String,
    score: u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<Student> {
        vec![
            Student {
                name: String::from("aa"),
                score: 60
            },
            Student {
                name: String::from("ca"),
                score: 50
            },
            Student {
                name: String::from("ab"),
                score: 80
            },
            Student {
                name: String::from("ca"),
                score: 10
            },
            Student {
                name: String::from("aa"),
                score: 90
            },
            Student {
                name: String::from("ba"),
                score: 20
            },
            Student {
                name: String::from("aa"),
                score: 80
            },
            Student {
                name: String::from("ab"),
                score: 90
            },
        ]
    }

    #[test]
    fn it_sort_1d() {
        let mut sample = get_sample();

        sample.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(sample, vec![
            Student {
                name: String::from("aa"),
                score: 60
            },
            Student {
                name: String::from("aa"),
                score: 90
            },
            Student {
                name: String::from("aa"),
                score: 80
            },
            Student {
                name: String::from("ab"),
                score: 80
            },
            Student {
                name: String::from("ab"),
                score: 90
            },
            Student {
                name: String::from("ba"),
                score: 20
            },
            Student {
                name: String::from("ca"),
                score: 50
            },
            Student {
                name: String::from("ca"),
                score: 10
            },
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
            Student {
                name: String::from("aa"),
                score: 60
            },
            Student {
                name: String::from("aa"),
                score: 80
            },
            Student {
                name: String::from("aa"),
                score: 90
            },
            Student {
                name: String::from("ab"),
                score: 80
            },
            Student {
                name: String::from("ab"),
                score: 90
            },
            Student {
                name: String::from("ba"),
                score: 20
            },
            Student {
                name: String::from("ca"),
                score: 10
            },
            Student {
                name: String::from("ca"),
                score: 50
            },
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
            Student {
                name: String::from("aa"),
                score: 60
            },
            Student {
                name: String::from("aa"),
                score: 80
            },
            Student {
                name: String::from("aa"),
                score: 90
            },
            Student {
                name: String::from("ab"),
                score: 80
            },
            Student {
                name: String::from("ab"),
                score: 90
            },
            Student {
                name: String::from("ba"),
                score: 20
            },
            Student {
                name: String::from("ca"),
                score: 10
            },
            Student {
                name: String::from("ca"),
                score: 50
            },
        ])
    }
}
