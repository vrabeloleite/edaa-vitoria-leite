pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
        assert_eq!(fibonacci_recursivo(6), 8);
    }
}
