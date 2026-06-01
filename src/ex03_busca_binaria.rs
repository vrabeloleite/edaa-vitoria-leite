pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;
    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busca_binaria() {
        let v = [10, 20, 30, 40, 50];
        assert_eq!(busca_binaria(&v, 30), Some(2));
    }
}
