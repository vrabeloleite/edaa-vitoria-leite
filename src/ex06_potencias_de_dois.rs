pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut i: u64 = 1;
    let mut lista = Vec::new();
    // multiplica por 2 ate dar o limite n
    while i < n {
        println!("{}", i);
        lista.push(i);
        i *= 2;
    }
    lista
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potencias() {
        assert_eq!(potencias_de_dois(10), vec![1, 2, 4, 8]);
    }
}
