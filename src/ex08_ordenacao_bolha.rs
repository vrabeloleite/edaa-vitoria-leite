pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut v = [5, 2, 4, 1];
        ordenacao_bolha(&mut v);
        assert_eq!(v, [1, 2, 4, 5]);
    }
}
