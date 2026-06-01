pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // bloco 1 printa sozinho
    for &x in lista {
        println!("{}", x);
    }

    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imprimir() {
        imprimir_pares_e_pares(&[1, 2]);
        assert!(true);
    }
}
