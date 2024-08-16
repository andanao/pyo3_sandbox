#[cfg(test)]
mod tests {
    use pyo3_test::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
