#[cfg(test)]
mod tests {
    use crate::utils::min;

    #[test]
    fn test_min() {
        let result = min(2.0, 3.0);
        assert_eq!(result, 2.0);
    }
}
