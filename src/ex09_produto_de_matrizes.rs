pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matriz() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![2, 0], vec![1, 2]];
        assert_eq!(produto_de_matrizes(&a, &b), vec![vec![4, 4], vec![10, 8]]);
    }
}
