#[cfg(test)]
mod tests {
    use crate::utils::max;

    #[test]
    fn test_min() {
        let result = max(2.0, 3.0);
        assert_eq!(result, 3.0);
    }
}
