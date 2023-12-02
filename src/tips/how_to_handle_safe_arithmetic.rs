pub fn addition_normal(a: u32, b: u32) -> u32 {
    a + b
}

pub fn addition_safe(a: u32, b: u32) -> Option<u32> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn addition_normal_overflow() {
        let a = u32::MAX;
        let b = 1;

        // attempt to add with overflow
        addition_normal(a, b);
    }

    #[test]
    fn addition_safe_overflow() {
        let a = u32::MAX;
        let b = 1;

        assert!(addition_safe(a, b).is_none());
    }
}
