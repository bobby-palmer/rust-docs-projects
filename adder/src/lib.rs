pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_2_to_4() {
        let result = add(2, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_that_will_fail() {
        let result = add(3, 4);
        assert_eq!(result, 6);
    }
}
