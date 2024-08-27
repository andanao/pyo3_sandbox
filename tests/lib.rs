#[cfg(test)]
mod tests {
    use pyo3_test::math_operations::add;

    #[test]
    fn test_add() {
        let result = add(2, 2).unwrap();
        assert_eq!(result, 4);
    }
}
