pub fn imprimir_pares_e_pares(lista: &[i32]) {
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
    fn test_imprimir_existencia() {
        imprimir_pares_e_pares(&[1, 2]);
        assert!(true);
    }
}
