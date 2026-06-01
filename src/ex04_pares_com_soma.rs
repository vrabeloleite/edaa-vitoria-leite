pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado = Vec::new();
    
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
                resultado.push((lista[i], lista[j]));
            }
        }
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pares_com_soma() {
        let pares = pares_com_soma(&[1, 2, 3, 4], 5);
        assert_eq!(pares, vec![(1, 4), (2, 3)]);
    }
}
