pub fn addition_normal(a: u32, b: u32) -> u32 {
    a + b
}

pub fn addition_safe(a: u32, b: u32) -> Option<u32> {
    a.checked_add(b)
}

pub fn subtraction_normal(a: u32, b: u32) -> u32 {
    a - b
}

pub fn subtraction_safe(a: u32, b: u32) -> Option<u32> {
    a.checked_sub(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn addition_normal_overflow() {
        let a = u32::MAX;
        let b = 1;

        addition_normal(a, b);
    }

    #[test]
    fn addition_safe_overflow() {
        let a = u32::MAX;
        let b = 1;

        assert!(addition_safe(a, b).is_none());
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn subtraction_normal_overflow() {
        let a = u32::MIN;
        let b = 1;

        subtraction_normal(a, b);
    }

    #[test]
    fn subtraction_safe_overflow() {
        let a = u32::MIN;
        let b = 1;

        assert!(subtraction_safe(a, b).is_none());
    }
}
