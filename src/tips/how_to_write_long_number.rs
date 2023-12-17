pub fn long_number_normal() -> i32 {
    let one_million_value = 1000000;

    println!("{}", one_million_value);

    one_million_value
}

pub fn long_number_using_separator() -> i32 {
    let one_million_value = 1_000_000;

    println!("{}", one_million_value);

    one_million_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_print() {
        assert!(long_number_normal() == long_number_using_separator());
    }
}
