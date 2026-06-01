/// Ordena um slice mutável utilizando o algoritmo Bubble Sort.
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
    fn test_ordenacao_bolha() {
        let mut vetor = [5, 3, 4, 1, 2];
        ordenacao_bolha(&mut vetor);
        assert_eq!(vetor, [1, 2, 3, 4, 5]);
    }
}
